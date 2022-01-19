use super::Gnbcu;
use crate::{ClientContext, F1ServerTransportProvider, NgapClientTransportProvider};
use also_net::{TnlaEvent, TnlaEventHandler};
use anyhow::Result;
use async_std::task::JoinHandle;
use async_trait::async_trait;
use common::f1ap::F1apPdu;
use node_control_api::Api;
use slog::{trace, warn, Logger};
use stop_token::StopToken;

impl<
        T: NgapClientTransportProvider,
        F: F1ServerTransportProvider,
        C: Api<ClientContext> + Send + Sync + Clone + 'static,
    > Gnbcu<T, F, C>
{
    pub async fn start_f1ap_handler(&self, stop_token: StopToken) -> Result<JoinHandle<()>> {
        let addr = format!("0.0.0.0:{}", self.config.callback_server_bind_port);
        let task = self
            .f1_transport_provider
            .clone()
            .serve(
                addr.to_string(),
                stop_token,
                self.clone(),
                self.logger.clone(),
            )
            .await?;
        Ok(task)
    }
}

#[async_trait]
impl<T, F, C> TnlaEventHandler<F1apPdu> for Gnbcu<T, F, C>
where
    T: NgapClientTransportProvider,
    F: F1ServerTransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
{
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        match event {
            TnlaEvent::Established => trace!(logger, "TNLA {} established", tnla_id),
            TnlaEvent::Terminated => warn!(logger, "TNLA {} closed", tnla_id),
        };
        unimplemented!()
    }

    async fn handle_message(&self, message: F1apPdu, _tnla_id: u32, logger: &Logger) {
        trace!(logger, "f1ap_pdu: {:?}", message);
        unimplemented!()
    }
}
