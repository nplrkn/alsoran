use crate::sctp::SctpAssociation;
use crate::transport_provider::{ClientTransportProvider, Handler, Message, TransportProvider};
use anyhow::{anyhow, Result};
use async_std::sync::Arc;
use async_std::sync::Mutex;
use async_std::task;
use async_trait::async_trait;
use os_socketaddr::OsSocketAddr;
use slog::{info, warn, Logger};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct SctpClientTransportProvider {
    assocs: Arc<Mutex<Box<HashMap<u32, Arc<SctpAssociation>>>>>,
    ppid: u32,
}

impl SctpClientTransportProvider {
    pub fn new(ppid: u32) -> SctpClientTransportProvider {
        let assocs = Arc::new(Mutex::new(Box::new(HashMap::new())));
        SctpClientTransportProvider { assocs, ppid }
    }
}

#[async_trait]
impl ClientTransportProvider for SctpClientTransportProvider {
    async fn maintain_connection<R: Handler>(
        &self,
        connect_addr_string: String,
        handler: R,
        logger: Logger,
    ) -> Result<()> {
        let shared_assocs = self.assocs.clone();
        let ppid = self.ppid.clone();

        task::spawn(async move {
            loop {
                let addr = async_net::resolve(connect_addr_string.clone())
                    .await
                    .map(|vec| vec[0])
                    .unwrap(); // TODO
                let addr: OsSocketAddr = addr.into();
                let assoc_id = 3; // TODO
                let assoc = SctpAssociation::establish(addr, ppid, &logger).await;

                let retry_duration = if let Ok(assoc) = assoc {
                    let assoc = Arc::new(assoc);
                    shared_assocs.lock().await.insert(assoc_id, assoc.clone());

                    // TODO Hack
                    // Instead we should send a notification about this connection being up
                    let precanned_ng_setup = hex::decode("00150035000004001b00080002f83910000102005240090300667265653567630066001000000000010002f839000010080102030015400140").unwrap();
                    assoc.send_msg(precanned_ng_setup).await.unwrap();

                    while let Ok(message) = assoc.recv_msg().await {
                        info!(
                            logger,
                            "Sctp client received {:?}, forward to handler", message
                        );
                        handler.recv_non_ue_associated(message, &logger).await;
                    }
                    warn!(logger, "SCTP connection terminated - 5s pause before retry");
                    5
                } else {
                    warn!(
                        logger,
                        "SCTP connection establish failure - 30s pause before retry"
                    );
                    30
                };
                shared_assocs.lock().await.remove(&assoc_id);
                task::sleep(Duration::from_secs(retry_duration)).await;
            }
        });
        Ok(())
    }
}

#[async_trait]
impl TransportProvider for SctpClientTransportProvider {
    async fn send_message(&self, message: Message, _logger: &Logger) -> Result<()> {
        if let Some(assoc) = self.assocs.lock().await.values().next() {
            Ok(assoc.send_msg(message).await?)
        } else {
            Err(anyhow!("No association up"))
        }
    }
    async fn start_receiving<R: Handler>(&self, _handler: R, _logger: &Logger) {
        unimplemented!();
    }
}
