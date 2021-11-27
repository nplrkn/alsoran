use crate::gnbcu::Gnbcu;
use crate::transport_provider::{Handler, Message, TransportProvider};
use async_trait::async_trait;
use slog::Logger;
use slog::{info, o};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct NgapHandler<T: TransportProvider, F: TransportProvider> {
    gnbcu: Arc<Gnbcu<T, F>>,
}

impl<T: TransportProvider, F: TransportProvider> NgapHandler<T, F> {
    pub fn new(gnbcu: Gnbcu<T, F>) -> NgapHandler<T, F> {
        NgapHandler {
            gnbcu: Arc::new(gnbcu),
        }
    }
}

#[async_trait]
impl<T, F> Handler for NgapHandler<T, F>
where
    T: TransportProvider,
    F: TransportProvider,
{
    async fn recv_non_ue_associated(&self, message: Message, logger: &Logger) {
        info!(
            logger,
            "NgapHandler got non UE associated message {:?} - forward to F1 transport", message
        );
        let logger = logger.new(o!("component" => "F1"));
        self.gnbcu
            .f1_transport_provider
            .send_message(message, &logger)
            .await
            .unwrap();
    }
}
