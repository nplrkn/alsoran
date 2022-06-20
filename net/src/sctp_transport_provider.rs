use super::sctp_tnla_pool::SctpTnlaPool;
use super::tnla_event_handler::TnlaEventHandler;
use crate::TransportProvider;
use anyhow::{anyhow, Result};
use async_std::sync::Arc;
use async_std::task;
use async_trait::async_trait;
use futures::stream::StreamExt;
use futures::{future, pin_mut};
use sctp::{Message, SctpAssociation};
use slog::{debug, info, o, warn, Logger};
use std::fmt::Debug;
use std::net::SocketAddr;
use std::time::Duration;
use stop_token::StopToken;
use task::JoinHandle;

#[derive(Debug, Clone)]
pub struct SctpTransportProvider {
    tnla_pool: SctpTnlaPool,
    ppid: u32,
}

impl SctpTransportProvider {
    pub fn new(ppid: u32) -> SctpTransportProvider {
        SctpTransportProvider {
            tnla_pool: SctpTnlaPool::new(),
            ppid,
        }
    }
}

async fn resolve_and_connect(
    connect_addr_string: &str,
    ppid: u32,
    logger: &Logger,
) -> Result<SctpAssociation> {
    let addr = async_net::resolve(connect_addr_string)
        .await?
        .into_iter()
        .next()
        .ok_or(anyhow!("Address resolved to empty array"))?; // Don't know if this is actually hittable
    SctpAssociation::establish(addr, ppid, logger).await
}

#[async_trait]
impl TransportProvider for SctpTransportProvider {
    async fn send_message(&self, message: Message, logger: &Logger) -> Result<()> {
        self.tnla_pool.send_message(message, logger).await
    }

    async fn maintain_connection<H>(
        self,
        connect_addr_string: String,
        stop_token: StopToken,
        handler: H,
        logger: Logger,
    ) -> Result<JoinHandle<()>>
    where
        H: TnlaEventHandler,
    {
        let assoc_id = 3; // TODO
        let task = task::spawn(async move {
            loop {
                match resolve_and_connect(&connect_addr_string, self.ppid, &logger).await {
                    Ok(assoc) => {
                        let logger = logger.new(o!("connection" => assoc_id));
                        debug!(logger, "Established connection");

                        self.tnla_pool
                            .add_and_handle_no_spawn(
                                assoc_id,
                                Arc::new(assoc),
                                handler.clone(),
                                stop_token.clone(),
                                logger.clone(),
                            )
                            .await;
                        warn!(logger, "SCTP connection terminated - will retry");
                    }
                    Err(e) => {
                        warn!(
                            logger,
                            "Couldn't establish connection to {} - will retry ({:?})",
                            connect_addr_string,
                            e
                        );
                    }
                };
                let retry_duration = Duration::from_secs(30);
                if async_std::future::timeout(retry_duration, stop_token.clone())
                    .await
                    .is_ok()
                {
                    break;
                }
            }
        });
        Ok(task)
    }

    // Return the set of TNLA remote address to which we are currently connected
    async fn remote_tnla_addresses(&self) -> Vec<SocketAddr> {
        self.tnla_pool.remote_addresses().await
    }

    async fn serve<H>(
        self,
        listen_addr: String,
        stop_token: StopToken,
        handler: H,
        logger: Logger,
    ) -> Result<JoinHandle<()>>
    where
        H: TnlaEventHandler,
    {
        let addr = async_net::resolve(listen_addr).await.map(|vec| vec[0])?;

        // let wrapped_handler = Wrapper {
        //     handler,
        //     codec: self.codec,
        // };

        let stream = sctp::new_listen(addr, self.ppid, MAX_LISTEN_BACKLOG, logger.clone())?;
        let stream = stream.take_until(stop_token.clone());

        Ok(task::spawn(async move {
            debug!(logger, "Listening for SCTP connections on {:?}", addr);
            pin_mut!(stream);
            let mut connection_tasks = vec![];
            loop {
                match stream.next().await {
                    Some(Ok(assoc)) => {
                        let assoc_id = 53; // TODO
                        let logger = logger.new(o!("connection" => assoc_id));
                        info!(
                            logger,
                            "Accepted SCTP connection from {}", assoc.remote_address
                        );
                        let task = self
                            .tnla_pool
                            .clone()
                            .add_and_handle(
                                assoc_id,
                                Arc::new(assoc),
                                handler.clone(),
                                stop_token.clone(),
                                logger,
                            )
                            .await;
                        connection_tasks.push(task);
                    }
                    Some(Err(e)) => warn!(logger, "Error on incoming connection - {:?}", e),
                    None => {
                        info!(logger, "Graceful shutdown");
                        break;
                    }
                }
            }

            debug!(logger, "Wait for connection tasks to finish");
            future::join_all(connection_tasks).await;
            debug!(logger, "Connection tasks finished");
        }))
    }
}

const MAX_LISTEN_BACKLOG: i32 = 5;
