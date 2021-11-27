use crate::gnbcu::Gnbcu;
use crate::transport_provider::{Handler, Message, TransportProvider};
use async_trait::async_trait;
use slog::Logger;
use slog::{info, o};
use std::sync::Arc;
pub struct F1Handler<T: TransportProvider, F: TransportProvider> {
    gnbcu: Arc<Gnbcu<T, F>>,
}

impl<T: TransportProvider, F: TransportProvider> F1Handler<T, F> {
    pub fn new(gnbcu: Gnbcu<T, F>) -> F1Handler<T, F> {
        F1Handler {
            gnbcu: Arc::new(gnbcu),
        }
    }
}

#[async_trait]
impl<T, F> Handler for F1Handler<T, F>
where
    T: TransportProvider,
    F: TransportProvider,
{
    async fn recv_non_ue_associated(&self, message: Message, logger: &Logger) {
        info!(
            logger,
            "F1Handler got non UE associated message {:?} - forward to NGAP transport", message
        );
        let logger = logger.new(o!("component" => "NGAP"));
        self.gnbcu
            .ngap_transport_provider
            .send_message(message, &logger)
            .await
            .unwrap();
    }
}
