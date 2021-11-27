use crate::sctp::SctpAssociation;
use crate::transport_provider::{ClientTransportProvider, Handler, Message, TransportProvider};
use async_trait::async_trait;
use slog::Logger;

#[derive(Debug, Clone)]
pub struct SctpClientTransportProvider {
    assocs: Vec<SctpAssociation>,
}

impl SctpClientTransportProvider {
    pub fn new() -> SctpClientTransportProvider {
        unimplemented!();
    }
}

#[async_trait]
impl ClientTransportProvider for SctpClientTransportProvider {
    async fn connect(&mut self, connect_addr_string: String) -> Result<(), String> {
        // TODO - this is not how we should deal with errors
        let address_list = async_net::resolve(connect_addr_string)
            .await
            .map_err(|_| "Didn't resolve")?;
        let first_address = address_list.get(0).ok_or("Didn't resolve")?;
        let assoc = SctpAssociation::establish(first_address)
            .await
            .map_err(|_| "Establishment failure")?;
        self.assocs = vec![assoc];
        Ok(())
    }
}

#[async_trait]
impl TransportProvider for SctpClientTransportProvider {
    async fn send_message(&self, _message: Message, _logger: &Logger) -> Result<(), String> {
        unimplemented!();
    }
    async fn start_receiving<R: Handler>(&self, _handler: R, _logger: &Logger) {
        unimplemented!();
    }
}
