use crate::gnbcu::Gnbcu;
use crate::{ClientContext, F1ServerTransportProvider, NgapClientTransportProvider};
use also_net::{TnlaEvent, TnlaEventHandler};
use async_trait::async_trait;
use common::ngap::*;
use node_control_api::Api;
use slog::Logger;
use slog::{trace, warn};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct NgapHandler<T, F, C>
where
    T: NgapClientTransportProvider,
    F: F1ServerTransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
{
    gnbcu: Arc<Gnbcu<T, F, C>>,
}

impl<T, F, C> NgapHandler<T, F, C>
where
    T: NgapClientTransportProvider,
    F: F1ServerTransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
{
    pub fn new(gnbcu: Gnbcu<T, F, C>) -> NgapHandler<T, F, C> {
        NgapHandler {
            gnbcu: Arc::new(gnbcu),
        }
    }
}

#[async_trait]
impl<T, F, C> TnlaEventHandler for NgapHandler<T, F, C>
where
    T: NgapClientTransportProvider,
    F: F1ServerTransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
{
    type MessageType = NgapPdu;

    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        match event {
            TnlaEvent::Established => {
                trace!(logger, "TNLA {} established", tnla_id);
                // TODO update coordinator
            }
            TnlaEvent::Terminated => warn!(logger, "TNLA {} closed", tnla_id),
        };
    }

    async fn handle_message(&self, message: NgapPdu, _tnla_id: u32, logger: &Logger) {
        trace!(logger, "ngap_pdu: {:?}", message);
    }
}
