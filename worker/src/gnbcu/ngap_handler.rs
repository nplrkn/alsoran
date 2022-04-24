use super::Gnbcu;
use crate::ClientContext;
use async_trait::async_trait;
use net::Message;
use net::{TnlaEvent, TnlaEventHandler, TransportProvider};
use node_control_api::Api;
use slog::{trace, warn, Logger};

#[derive(Clone)]
struct NgapHandler<N, F, C>
where
    N: TransportProvider,
    F: TransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
{
    gnbcu: Gnbcu<N, F, C>,
}

#[async_trait]
impl<N, F, C> TnlaEventHandler for NgapHandler<N, F, C>
where
    N: TransportProvider,
    F: TransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
{
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        match event {
            TnlaEvent::Established => trace!(logger, "NGAP TNLA {} established", tnla_id),
            TnlaEvent::Terminated => warn!(logger, "NGAP TNLA {} closed", tnla_id),
        };
        self.gnbcu.connected_amf_change(logger).await;
    }

    async fn handle_message(&self, message: Message, _tnla_id: u32, logger: &Logger) {
        trace!(logger, "ngap_pdu: {:?}", message);
    }
}
