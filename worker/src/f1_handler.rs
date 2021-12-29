use crate::gnbcu::Gnbcu;
use crate::{ClientContext, F1ServerTransportProvider, NgapClientTransportProvider};
use also_net::{TnlaEvent, TnlaEventHandler};
use async_trait::async_trait;
use common::ngap::NgapPdu;
use node_control_api::Api;
use slog::Logger;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct F1Handler<
    T: NgapClientTransportProvider,
    F: F1ServerTransportProvider,
    C: Api<ClientContext> + Sync + Send + Clone + 'static,
> {
    gnbcu: Arc<Gnbcu<T, F, C>>,
}

impl<
        T: NgapClientTransportProvider,
        F: F1ServerTransportProvider,
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
impl<T, F, C> TnlaEventHandler for F1Handler<T, F, C>
where
    T: NgapClientTransportProvider,
    F: F1ServerTransportProvider,
    C: Api<ClientContext> + Sync + Send + 'static + Clone,
{
    type MessageType = NgapPdu; // TODO

    async fn handle_event(&self, _event: TnlaEvent, _tnla_id: u32, _logger: &Logger) {
        unimplemented!()
    }

    // TODO indicate whether it is UE or non UE associated?
    async fn handle_message(&self, _message: NgapPdu, _tnla_id: u32, _logger: &Logger) {
        unimplemented!()
    }
}
