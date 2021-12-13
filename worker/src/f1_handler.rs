use crate::gnbcu::Gnbcu;
use crate::transport_provider::{ClientTransportProvider, Handler, Message, TransportProvider};
use crate::ClientContext;
use async_trait::async_trait;
use node_control_api::Api;
use slog::Logger;
use slog::{info, o};
use std::sync::Arc;

pub struct F1Handler<
    T: ClientTransportProvider,
    F: TransportProvider,
    C: Api<ClientContext> + Sync + Send + Clone + 'static,
> {
    gnbcu: Arc<Gnbcu<T, F, C>>,
}

impl<
        T: ClientTransportProvider,
        F: TransportProvider,
        C: Api<ClientContext> + Sync + Send + Clone,
    > F1Handler<T, F, C>
{
    pub fn new(gnbcu: Gnbcu<T, F, C>) -> F1Handler<T, F, C> {
        F1Handler {
            gnbcu: Arc::new(gnbcu),
        }
    }
}

#[async_trait]
impl<T, F, C> Handler for F1Handler<T, F, C>
where
    T: ClientTransportProvider,
    F: TransportProvider,
    C: Api<ClientContext> + Sync + Send + 'static + Clone,
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
