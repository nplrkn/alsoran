use super::Gnbcu;
use crate::{ClientContext, F1ServerTransportProvider, NgapClientTransportProvider};
use async_trait::async_trait;
use net::{TnlaEvent, TnlaEventHandler};
use ngap::NgapPdu;
use node_control_api::Api;
use slog::{trace, warn, Logger};

#[async_trait]
impl<T, F, C> TnlaEventHandler<NgapPdu> for Gnbcu<T, F, C>
where
    T: NgapClientTransportProvider,
    F: F1ServerTransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
{
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        match event {
            TnlaEvent::Established => trace!(logger, "NGAP TNLA {} established", tnla_id),
            TnlaEvent::Terminated => warn!(logger, "NGAP TNLA {} closed", tnla_id),
        };
        self.connected_amf_change(logger).await;
    }

    async fn handle_message(&self, message: NgapPdu, _tnla_id: u32, logger: &Logger) {
        trace!(logger, "ngap_pdu: {:?}", message);
    }
}
