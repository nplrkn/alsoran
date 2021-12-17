use crate::gnbcu::Gnbcu;
use crate::ClientContext;
use async_trait::async_trait;
use common::transport_provider::{ClientTransportProvider, Handler, Message, TransportProvider};
use node_control_api::Api;
use slog::info;
use slog::Logger;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct NgapHandler<T, F, C>
where
    T: ClientTransportProvider,
    F: TransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
{
    gnbcu: Arc<Gnbcu<T, F, C>>,
}

impl<T, F, C> NgapHandler<T, F, C>
where
    T: ClientTransportProvider,
    F: TransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
{
    pub fn new(gnbcu: Gnbcu<T, F, C>) -> NgapHandler<T, F, C> {
        NgapHandler {
            gnbcu: Arc::new(gnbcu),
        }
    }
}

#[async_trait]
impl<T, F, C> Handler for NgapHandler<T, F, C>
where
    T: ClientTransportProvider,
    F: TransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
{
    async fn recv_non_ue_associated(&self, message: Message, logger: &Logger) {
        info!(
            logger,
            "NgapHandler got non UE associated message {:?}", message
        );
        // self.gnbcu
        //     .f1_transport_provider
        //     .send_message(message, &logger)
        //     .await
        //     .unwrap();
    }
}
