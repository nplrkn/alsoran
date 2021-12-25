use anyhow::Result;
use async_std::task::JoinHandle;
use async_trait::async_trait;
use slog::Logger;
use stop_token::StopToken;

//pub struct Binding;

pub type Message = Vec<u8>;

/// The TransportProvider trait allows the user to send UE and non-UE-associated messages over
/// some reference point without needing to understand the details of transport connections.  
#[async_trait]
pub trait TransportProvider: 'static + Send + Sync + Clone {
    async fn send_message(&self, message: Message, logger: &Logger) -> Result<()>;
}

/// The Handler trait covers the function needed to receive messages over a transport.
#[async_trait]
pub trait Handler: 'static + Send + Sync + Clone {
    async fn tnla_established(&self, tnla_id: u32, logger: &Logger);
    async fn tnla_terminated(&self, tnla_id: u32, logger: &Logger);
    async fn recv_non_ue_associated(&self, m: Message, logger: &Logger);
}

/// The ServerTransportProvider trait provides the functions needed on the passive
/// side of the reference point (which accepts connections from the active side).
#[async_trait]
pub trait ServerTransportProvider: TransportProvider {
    async fn serve<H>(
        &self,
        listen_addr: String,
        stop_token: StopToken,
        hander: H,
        logger: Logger,
    ) -> Result<JoinHandle<()>>
    where
        H: Handler;
}

/// The ClientTransportProvider trait provides the functions needed on the active
/// side of the reference point (which initiates connections towards the passive side).
#[async_trait]
pub trait ClientTransportProvider: TransportProvider {
    // TODO Eventually this will evolve into add_connection_target (?)
    async fn maintain_connection<H: Handler>(
        self,
        connect_addr_string: String,
        handler: H,
        stop_token: StopToken,
        logger: Logger,
    ) -> Result<JoinHandle<()>>;
}
