use crate::transport_provider::{ClientTransportProvider, Handler, Message, TransportProvider};
use async_trait::async_trait;
use slog::Logger;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct SctpClientTransportProvider {}

impl SctpClientTransportProvider {
    pub fn new() -> SctpClientTransportProvider {
        unimplemented!();
    }
}

#[async_trait]
impl ClientTransportProvider for SctpClientTransportProvider {
    async fn connect(connect_addr_string: String) -> Result<(), String> {
        let address_list = async_net::resolve(connect_addr_string).await.unwrap();
        unimplemented!();
    }
}

#[async_trait]
impl TransportProvider for SctpClientTransportProvider {
    async fn send_message(&self, message: Message, logger: &Logger) -> Result<(), String> {
        unimplemented!();
    }
    async fn start_receiving<R: Handler>(&self, handler: R, logger: &Logger) {
        unimplemented!();
    }
}
