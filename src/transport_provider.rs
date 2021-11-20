/// Transport provider
use async_trait::async_trait;
use slog::Logger;

//pub struct Binding;

pub type Message = Vec<u8>;

#[async_trait]
pub trait TransportProvider: 'static + Send + Sync + Clone {
    async fn send_message(&self, message: Message, logger: &Logger) -> Result<(), String>;
    async fn start_receiving<R: Handler>(&self, handler: R, logger: &Logger);
}

#[async_trait]
pub trait Handler: 'static + Send + Sync {
    async fn recv_non_ue_associated(&self, m: Message, logger: &Logger);
}

// #[async_trait]
// pub trait ServerTransportProvider {
//     async fn listen<A: AsyncToSocketAddrs>(listen_addr: A) -> Result<Self, String>;
// }

// #[async_trait]
// pub trait ClientTransportProvider {
//     async fn connect<A: AsyncToSocketAddrs>(connect_addr: A) -> Result<Self, String>;
// }
