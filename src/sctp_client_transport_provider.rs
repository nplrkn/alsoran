use crate::sctp::SctpAssociation;
use crate::transport_provider::{ClientTransportProvider, Handler, Message, TransportProvider};
use async_trait::async_trait;
use slog::{info, Logger};

#[derive(Debug, Clone)]
pub struct SctpClientTransportProvider {
    //assocs: Vec<SctpAssociation>,
}

impl SctpClientTransportProvider {
    pub fn new() -> SctpClientTransportProvider {
        SctpClientTransportProvider {}
    }
}

#[async_trait]
impl ClientTransportProvider for SctpClientTransportProvider {
    async fn connect<R: Handler>(
        &mut self,
        connect_addr_string: String,
        handler: R,
        logger: Logger,
    ) -> Result<(), String> {
        // TODO - this is not how we should deal with errors
        let address_list = async_net::resolve(connect_addr_string)
            .await
            .map_err(|_| "Didn't resolve")?;
        let first_address = address_list.get(0).ok_or("Didn't resolve")?;
        let assoc = SctpAssociation::establish(first_address, &logger)
            .await
            .map_err(|e| e.to_string())?;

        //    self.assocs = vec![assoc];

        async_std::task::spawn(async move {
            while let Ok(message) = assoc.recv_msg().await {
                info!(
                    logger,
                    "Sctp client received {:?}, forward to handler", message
                );
                handler.recv_non_ue_associated(message, &logger).await;
            }
        });

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
