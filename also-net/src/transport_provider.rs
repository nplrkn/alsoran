use crate::tnla_event_handler::TnlaEventHandler;
use anyhow::Result;
use async_std::task::JoinHandle;
use async_trait::async_trait;
use slog::Logger;
use std::net::SocketAddr;
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
// TODO - change this to not need handler as a parameter but have self impl handler?
#[async_trait]
pub trait ServerTransportProvider<P>: TransportProvider {
    async fn serve<H>(
        self,
        listen_addr: String,
        stop_token: StopToken,
        handler: H,
        logger: Logger,
    ) -> Result<JoinHandle<()>>
    where
        H: TnlaEventHandler<P>;
}

/// The ClientTransportProvider trait provides the functions needed on the active
/// side of the reference point (which initiates connections towards the passive side).
#[async_trait]
pub trait ClientTransportProvider<Pdu>: TransportProvider {
    // TODO Eventually this will evolve into add_tnla_address (?)
    async fn maintain_connection<H>(
        self,
        connect_addr_string: String,
        handler: H,
        stop_token: StopToken,
        logger: Logger,
    ) -> Result<JoinHandle<()>>
    where
        H: TnlaEventHandler<Pdu>;

    // Return the set of TNLA remote address to which we are currently connected
    async fn remote_tnla_addresses(&self) -> Vec<SocketAddr>;
}
