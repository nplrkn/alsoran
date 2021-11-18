/// Transport provider
use async_net::AsyncToSocketAddrs;
use async_std::sync::Arc;
use async_trait::async_trait;

pub struct Binding;

#[async_trait]
pub trait TransportMessageReceiver: Send + Clone {
    /// Receive a non UE associated message.
    async fn recv_non_ue_associated_message(&self, buf: Vec<u8>);

    //async fn recv_ue_associated_message(&self, buf: &[u8], tnla_binding: Binding);
}

#[async_trait]
pub trait TransportProvider {
    type Receiver;
    async fn set_receiver(&mut self, receiver: Self::Receiver);
    async fn send_non_ue_associated_message(&self, buf: &[u8]) -> Result<usize, String>;
    //async fn send_ue_associated(&self, buf: &[u8], requested_ue_tnla_binding: Binding) -> Result<(usize, Binding)>
}

// #[async_trait]
// pub trait ServerTransportProvider {
//     async fn listen<A: AsyncToSocketAddrs>(listen_addr: A) -> Result<Self, String>;
// }

// #[async_trait]
// pub trait ClientTransportProvider {
//     async fn connect<A: AsyncToSocketAddrs>(connect_addr: A) -> Result<Self, String>;
// }
