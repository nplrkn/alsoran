use crate::sctp::SctpListener;
use crate::sctp_tnla_pool::SctpTnlaPool;
use crate::transport_provider::{Handler, Message, ServerTransportProvider, TransportProvider};
use anyhow::Result;
use async_std::prelude::Future;
use async_std::sync::Arc;
use async_std::task;
use async_std::task::JoinHandle;
use async_trait::async_trait;
use os_socketaddr::OsSocketAddr;
use slog::{info, o, Logger};

// TODO common structure with the client version
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

// TODO share implementation with sctp_client_transport_provider
#[async_trait]
impl TransportProvider for SctpServerTransportProvider {
    async fn send_message(&self, message: Message, logger: &Logger) -> Result<()> {
        self.tnla_pool.send_message(message, logger).await
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
        let listener = SctpListener::bind(addr, self.ppid, logger.clone())?;
        let tnla_pool = self.tnla_pool.clone();
        Ok(task::spawn(async move {
            info!(logger, "Listening for connections");
            while let Ok(assoc) = listener.accept_next().await {
                let assoc_id = 53; // TODO
                let logger = logger.new(o!("connection" => assoc_id));
                info!(logger, "Accepted connection");
                let _task = tnla_pool
                    .add_and_handle(assoc_id, Arc::new(assoc), handler.clone(), logger)
                    .await;
            }
        }))
    }
}
