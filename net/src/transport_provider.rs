use crate::{tnla_event_handler::TnlaEventHandler, ShutdownHandle};
use anyhow::Result;
use async_net::SocketAddr;
use async_trait::async_trait;
use sctp::Message;
use slog::Logger;

//pub struct Binding;

/// The TransportProvider trait abstracts the transport, for example, to allow a non-SCTP test transport to be used.
#[async_trait]
pub trait TransportProvider: Send + Sync + 'static {
    async fn send_message(&self, message: Message, logger: &Logger) -> Result<()>;

    async fn serve<H>(
        self,
        listen_addr: String,
        ppid: u32,
        handler: H,
        logger: Logger,
    ) -> Result<ShutdownHandle>
    where
        H: TnlaEventHandler;

    async fn maintain_connection<H>(
        self,
        connect_addr_string: String,
        ppid: u32,
        handler: H,
        logger: Logger,
    ) -> Result<ShutdownHandle>
    where
        H: TnlaEventHandler;

    // Return the set of TNLA remote address to which we are currently connected
    async fn remote_tnla_addresses(&self) -> Vec<SocketAddr>;
}
