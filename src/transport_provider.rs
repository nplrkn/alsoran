/// Transport provider
use async_trait::async_trait;

//pub struct Binding;

pub type Message = Vec<u8>;

#[async_trait]
pub trait TransportProvider: 'static + Send + Sync + Clone {
    async fn send_message(&self, message: Message) -> Result<(), String>;
    async fn recv_message(&self) -> Option<Message>;
}

// #[async_trait]
// pub trait ServerTransportProvider {
//     async fn listen<A: AsyncToSocketAddrs>(listen_addr: A) -> Result<Self, String>;
// }

// #[async_trait]
// pub trait ClientTransportProvider {
//     async fn connect<A: AsyncToSocketAddrs>(connect_addr: A) -> Result<Self, String>;
// }
