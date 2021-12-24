use super::sctp_tnla_pool::SctpTnlaPool;
use crate::sctp::SctpAssociation;
use crate::transport_provider::{ClientTransportProvider, Handler, Message, TransportProvider};
use anyhow::{anyhow, Result};
use async_std::sync::Arc;
use async_std::task;
use async_trait::async_trait;
use slog::{info, o, warn, Logger};
use std::time::Duration;
use stop_token::StopToken;
use task::JoinHandle;

#[derive(Debug, Clone)]
pub struct SctpClientTransportProvider {
    tnla_pool: SctpTnlaPool,
    ppid: u32,
}

impl SctpClientTransportProvider {
    pub fn new(ppid: u32) -> SctpClientTransportProvider {
        SctpClientTransportProvider {
            tnla_pool: SctpTnlaPool::new(),
            ppid,
        }
    }
}

async fn resolve_and_connect(
    connect_addr_string: &String,
    ppid: u32,
    logger: &Logger,
) -> Result<SctpAssociation> {
    let addr = async_net::resolve(connect_addr_string)
        .await?
        .into_iter()
        .nth(0)
        .ok_or(anyhow!("Address resolved to empty array"))? // Don't know if this is actually hittable
        .into();
    SctpAssociation::establish(addr, ppid, logger).await
}

#[async_trait]
impl ClientTransportProvider for SctpClientTransportProvider {
    async fn maintain_connection<R: Handler>(
        &self,
        connect_addr_string: String,
        handler: R,
        stop_token: StopToken,
        logger: Logger,
    ) -> Result<JoinHandle<()>> {
        let tnla_pool = self.tnla_pool.clone();
        let ppid = self.ppid;
        let assoc_id = 3; // TODO

        let task = task::spawn(async move {
            loop {
                match resolve_and_connect(&connect_addr_string, ppid, &logger).await {
                    Ok(assoc) => {
                        let logger = logger.new(o!("connection" => assoc_id));
                        info!(logger, "Established connection");

                        let connection_handler = tnla_pool
                            .add_and_handle(
                                assoc_id,
                                Arc::new(assoc),
                                handler.clone(),
                                stop_token.clone(),
                                logger.clone(),
                            )
                            .await;
                        connection_handler.await;
                        warn!(logger, "SCTP connection terminated - will retry");
                    }
                    Err(e) => {
                        warn!(
                            logger,
                            "Couldn't establish connection - will retry ({:?})", e
                        );
                    }
                };
                let retry_duration = 30;
                task::sleep(Duration::from_secs(retry_duration)).await;
            }
        });
        Ok(task)
    }
}

#[async_trait]
impl TransportProvider for SctpClientTransportProvider {
    async fn send_message(&self, message: Message, logger: &Logger) -> Result<()> {
        self.tnla_pool.send_message(message, logger).await
    }
}
