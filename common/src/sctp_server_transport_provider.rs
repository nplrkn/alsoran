use crate::sctp::SctpListener;
use crate::transport_provider::{Handler, ServerTransportProvider};
use anyhow::Result;
use async_std::prelude::Future;
use async_std::stream::StreamExt;
use async_std::task;
use async_std::task::JoinHandle;
use async_trait::async_trait;
use slog::{info, o, trace, Logger};

#[derive(Debug, Clone)]
pub struct SctpServerTransportProvider;

impl SctpServerTransportProvider {
    pub fn new() -> SctpServerTransportProvider {
        SctpServerTransportProvider {}
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
        let listener = SctpListener::bind(listen_addr).await?;
        Ok(task::spawn(async move {
            while let Some(Ok(assoc)) = listener.incoming().next().await {
                info!(logger, "Accepted new connection");
                let connection_logger = logger.new(o!("connection" => 1));
                let handler_clone = handler.clone();
                task::spawn(async move {
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
                });
            }
        }))
    }
}
