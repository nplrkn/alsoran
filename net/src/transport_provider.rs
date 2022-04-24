use crate::tnla_event_handler::TnlaEventHandler;
use anyhow::Result;
use async_net::SocketAddr;
use async_std::task::JoinHandle;
use async_trait::async_trait;
use sctp::Message;
use slog::Logger;
use stop_token::StopToken;

//pub struct Binding;

// TODO: Message should be a byte array slice or a Vec<u8>?

/// The TransportProvider trait abstracts the transport, for example, to allow a non-SCTP test transport to be used.
#[async_trait]
pub trait TransportProvider: Clone + Send + Sync + 'static {
    async fn send_message(&self, message: Message, logger: &Logger) -> Result<()>;

    async fn serve<H>(
        self,
        listen_addr: String,
        stop_token: StopToken,
        handler: H,
        logger: Logger,
    ) -> Result<JoinHandle<()>>
    where
        H: TnlaEventHandler;

    async fn maintain_connection<H>(
        self,
        connect_addr_string: String,
        handler: H,
        stop_token: StopToken,
        logger: Logger,
    ) -> Result<JoinHandle<()>>
    where
        H: TnlaEventHandler;

    // Return the set of TNLA remote address to which we are currently connected
    async fn remote_tnla_addresses(&self) -> Vec<SocketAddr>;
}
