use crate::sctp::SctpListener;
use crate::sctp_tnla_pool::SctpTnlaPool;
use crate::transport_provider::{Handler, Message, ServerTransportProvider, TransportProvider};
use anyhow::Result;
use async_std::sync::Arc;
use async_std::task;
use async_std::task::JoinHandle;
use async_trait::async_trait;
use futures::future;
use futures::{pin_mut, FutureExt};
use os_socketaddr::OsSocketAddr;
use slog::{info, o, trace, warn, Logger};
use stop_token::StopToken;

#[derive(Debug, Clone)]
pub struct SctpServerTransportProvider {
    tnla_pool: SctpTnlaPool,
    ppid: u32,
}

impl SctpServerTransportProvider {
    pub fn new(ppid: u32) -> SctpServerTransportProvider {
        SctpServerTransportProvider {
            tnla_pool: SctpTnlaPool::new(),
            ppid,
        }
    }
}

#[async_trait]
impl TransportProvider for SctpServerTransportProvider {
    async fn send_message(&self, message: Message, logger: &Logger) -> Result<()> {
        self.tnla_pool.send_message(message, logger).await
    }
}

const MAX_LISTEN_BACKLOG: i32 = 5;

#[async_trait]
impl ServerTransportProvider for SctpServerTransportProvider {
    async fn serve<H>(
        &self,
        listen_addr: String,
        stop_token: StopToken,
        handler: H,
        logger: Logger,
    ) -> Result<JoinHandle<()>>
    where
        H: Handler,
    {
        let addr = async_net::resolve(listen_addr.clone())
            .await
            .map(|vec| vec[0])?;
        let addr: OsSocketAddr = addr.into();
        let listener =
            SctpListener::new_listen(addr, self.ppid, MAX_LISTEN_BACKLOG, logger.clone())?;
        let tnla_pool = self.tnla_pool.clone();
        Ok(task::spawn(async move {
            info!(logger, "Listening for connections");
            let mut connection_tasks = vec![];
            let cloned_stop_token = stop_token.clone();
            let fused_stop_token = stop_token.fuse();
            pin_mut!(fused_stop_token);
            loop {
                let next = listener.accept_next().fuse();
                pin_mut!(next);
                futures::select! {
                    assoc = next => {
                        match assoc {
                            Ok(assoc) => {
                                let assoc_id = 53; // TODO
                                let logger = logger.new(o!("connection" => assoc_id));
                                info!(logger, "Accepted connection");
                                let task = tnla_pool
                                    .add_and_handle(assoc_id, Arc::new(assoc), handler.clone(), cloned_stop_token.clone(), logger)
                                    .await;
                                connection_tasks.push(task);
                            },
                            Err(e) => warn!(logger, "Accept connection failed - {:?}", e)
                        }
                    },
                    () = fused_stop_token => break
                }
            }
            info!(logger, "Graceful shutdown");
            trace!(logger, "Wait for connection tasks to finish");
            future::join_all(connection_tasks).await;
            trace!(logger, "Connection tasks finished");
        }))
    }
}
