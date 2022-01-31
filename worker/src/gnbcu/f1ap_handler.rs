use super::Gnbcu;
use crate::{ClientContext, F1ServerTransportProvider, NgapClientTransportProvider};
use also_net::{TnlaEvent, TnlaEventHandler};
use anyhow::Result;
use async_std::task::JoinHandle;
use async_trait::async_trait;
use f1ap::*;
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
        let addr = format!("0.0.0.0:{}", self.config.f1ap_bind_port);
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
            TnlaEvent::Established => trace!(logger, "F1AP TNLA {} established", tnla_id),
            TnlaEvent::Terminated => warn!(logger, "F1AP TNLA {} closed", tnla_id),
        };
    }

    async fn handle_message(&self, message: F1apPdu, _tnla_id: u32, logger: &Logger) {
        trace!(logger, "f1ap_pdu: {:?}", message);
        if let Some(response) = match message {
            F1apPdu::F1SetupRequest(x) => Some(self.f1_setup(x, logger).await.into()),
            x => {
                warn!(self.logger, "Unexpected or unhandled PDU {:?}", x);
                None
            }
        } {
            self.f1_transport_provider
                .send_pdu(response, logger) // include tnla id in future
                .await
                .unwrap_or_else(|e| warn!(self.logger, "Failed to send response {:?}", e));
        }
    }
}

impl<
        T: NgapClientTransportProvider,
        F: F1ServerTransportProvider,
        C: Api<ClientContext> + Send + Sync + Clone + 'static,
    > Gnbcu<T, F, C>
{
    async fn f1_setup(&self, message: F1SetupRequest, _logger: &Logger) -> F1SetupResponse {
        F1SetupResponse {
            transaction_id: message.transaction_id,
            gnb_cu_rrc_version: RrcVersion::new(16, 6, 0),
        }
    }
}
