use crate::tnla_event_handler::TnlaEventHandler;
use anyhow::Result;
use async_std::task::JoinHandle;
use async_trait::async_trait;
use slog::Logger;
use stop_token::StopToken;

//pub struct Binding;

/// The TransportProvider trait allows the user to send UE and non-UE-associated messages over
/// some reference point without needing to understand the details of transport connections.  
#[async_trait]
pub trait TransportProvider: 'static + Send + Sync + Clone {
    type Pdu;
    async fn send_pdu(&self, pdu: Self::Pdu, logger: &Logger) -> Result<()>;
}

/// The ServerTransportProvider trait provides the functions needed on the passive
/// side of the reference point (which accepts connections from the active side).
#[async_trait]
pub trait ServerTransportProvider: TransportProvider {
    type Pdu;
    async fn serve<H>(
        self,
        listen_addr: String,
        stop_token: StopToken,
        handler: H,
        logger: Logger,
    ) -> Result<JoinHandle<()>>
    where
        H: TnlaEventHandler<MessageType = <Self as ServerTransportProvider>::Pdu>;
}

/// The ClientTransportProvider trait provides the functions needed on the active
/// side of the reference point (which initiates connections towards the passive side).
#[async_trait]
pub trait ClientTransportProvider: TransportProvider {
    type Pdu;

    // TODO Eventually this will evolve into add_connection_target (?)
    async fn maintain_connection<H>(
        self,
        connect_addr_string: String,
        handler: H,
        stop_token: StopToken,
        logger: Logger,
    ) -> Result<JoinHandle<()>>
    where
        H: TnlaEventHandler<MessageType = <Self as ClientTransportProvider>::Pdu>;
}
