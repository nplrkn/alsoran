use anyhow::Result;
use async_std::prelude::Future;
use async_std::task::JoinHandle;
use async_trait::async_trait;
use slog::Logger;

//pub struct Binding;

pub type Message = Vec<u8>;

#[async_trait]
pub trait TransportProvider: 'static + Send + Sync + Clone {
    async fn send_message(&self, message: Message, logger: &Logger) -> Result<()>;
}

#[async_trait]
pub trait Handler: 'static + Send + Sync + Clone {
    async fn recv_non_ue_associated(&self, m: Message, logger: &Logger);
}

#[async_trait]
pub trait ServerTransportProvider: TransportProvider {
    async fn serve<F, H>(
        &self,
        listen_addr: String,
        graceful_shutdown_signal: F,
        hander: H,
        logger: Logger,
    ) -> Result<JoinHandle<()>>
    where
        F: Future<Output = ()> + Send + Sync,
        H: Handler;
}

#[async_trait]
pub trait ClientTransportProvider: TransportProvider {
    // TODO Eventually this will evolve into add_connection_target (?)
    async fn maintain_connection<H: Handler>(
        &self,
        connect_addr_string: String,
        handler: H,
        logger: Logger,
    ) -> Result<JoinHandle<()>>;
}
