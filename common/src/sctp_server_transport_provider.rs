use crate::sctp::SctpAssociation;
use crate::sctp::SctpListener;
use crate::transport_provider::{Handler, Message, ServerTransportProvider, TransportProvider};
use anyhow::anyhow;
use anyhow::Result;
use async_std::prelude::Future;
use async_std::sync::{Arc, Mutex};
use async_std::task;
use async_std::task::JoinHandle;
use async_trait::async_trait;
use os_socketaddr::OsSocketAddr;
use slog::{info, o, trace, Logger};
use std::collections::HashMap;

// TODO common structure with the client version
type SharedAssocHash = Arc<Mutex<Box<HashMap<u32, Arc<SctpAssociation>>>>>;

#[derive(Debug, Clone)]
pub struct SctpServerTransportProvider {
    assocs: SharedAssocHash,
    ppid: u32,
}

impl SctpServerTransportProvider {
    pub fn new(ppid: u32) -> SctpServerTransportProvider {
        let assocs = Arc::new(Mutex::new(Box::new(HashMap::new())));
        SctpServerTransportProvider { assocs, ppid }
    }
}

// TODO share implementation with sctp_client_transport_provider
#[async_trait]
impl TransportProvider for SctpServerTransportProvider {
    async fn send_message(&self, message: Message, _logger: &Logger) -> Result<()> {
        if let Some(assoc) = self.assocs.lock().await.values().next() {
            Ok(assoc.send_msg(message).await?)
        } else {
            Err(anyhow!("No association up"))
        }
    }
}

#[async_trait]
impl ServerTransportProvider for SctpServerTransportProvider {
    async fn serve<F, H>(
        &self,
        listen_addr: String,
        _graceful_shutdown_signal: F,
        handler: H,
        logger: Logger,
    ) -> Result<JoinHandle<()>>
    where
        F: Future<Output = ()> + Send + Sync,
        H: Handler,
    {
        let addr = async_net::resolve(listen_addr.clone())
            .await
            .map(|vec| vec[0])
            .unwrap(); // TODO
        let addr: OsSocketAddr = addr.into();
        let shared_assocs = self.assocs.clone();

        let listener = SctpListener::bind(addr, self.ppid, logger.clone())?;
        Ok(task::spawn(async move {
            info!(logger, "Listening for connections");
            //while let Some(assoc) = listener.next().await {
            while let Ok(assoc) = listener.accept_next().await {
                let assoc = Arc::new(assoc);
                info!(logger, "Accepted new connection");
                let assoc_id = 53; // TODO
                let shared_assocs = shared_assocs.clone();
                let connection_logger = logger.new(o!("connection" => 1));
                let handler_clone = handler.clone();
                task::spawn(async move {
                    shared_assocs.lock().await.insert(assoc_id, assoc.clone());
                    while let Ok(message) = assoc.recv_msg().await {
                        trace!(
                            connection_logger,
                            "Received {:?}, forward to handler",
                            message
                        );
                        handler_clone
                            .recv_non_ue_associated(message, &connection_logger)
                            .await;
                    }
                    info!(connection_logger, "Connection terminated");
                    shared_assocs.lock().await.remove(&assoc_id);
                });
            }
        }))
    }
}
