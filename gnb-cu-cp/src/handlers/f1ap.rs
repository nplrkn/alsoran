//! f1ap - F1AP entry points into the GNB-CU

use super::GnbCuCp;
use super::RrcHandler;
use crate::workflows::Workflow;
use async_trait::async_trait;
use f1ap::*;
use net::{
    EventHandler, IndicationHandler, RequestError, RequestProvider, ResponseAction, TnlaEvent,
};
use pdcp::PdcpPdu;
use slog::{debug, info, warn, Logger};

#[derive(Clone)]
pub struct F1apHandler<G: GnbCuCp> {
    gnb_cu_cp: G,
    rrc_handler: RrcHandler<G>,
}

impl<G: GnbCuCp> F1apHandler<G> {
    pub fn new_f1ap_application(
        gnb_cu_cp: G,
        rrc_handler: RrcHandler<G>,
    ) -> F1apCu<F1apHandler<G>> {
        F1apCu::new(F1apHandler {
            gnb_cu_cp,
            rrc_handler,
        })
    }
}

#[async_trait]
impl<G: GnbCuCp> RequestProvider<F1SetupProcedure> for F1apHandler<G> {
    async fn request(
        &self,
        r: F1SetupRequest,
        logger: &Logger,
    ) -> Result<ResponseAction<F1SetupResponse>, RequestError<F1SetupFailure>> {
        Workflow::new(&self.gnb_cu_cp, logger).f1_setup(r).await
    }
}

#[async_trait]
impl<G: GnbCuCp> RequestProvider<GnbDuConfigurationUpdateProcedure> for F1apHandler<G> {
    async fn request(
        &self,
        r: GnbDuConfigurationUpdate,
        logger: &Logger,
    ) -> Result<
        ResponseAction<GnbDuConfigurationUpdateAcknowledge>,
        RequestError<GnbDuConfigurationUpdateFailure>,
    > {
        Workflow::new(&self.gnb_cu_cp, logger)
            .gnb_du_configuration_update(r)
            .await
    }
}

#[async_trait]
impl<G: GnbCuCp> IndicationHandler<InitialUlRrcMessageTransferProcedure> for F1apHandler<G> {
    async fn handle(&self, r: InitialUlRrcMessageTransfer, logger: &Logger) {
        if let Err(e) = Workflow::new(&self.gnb_cu_cp, logger)
            .initial_access(r)
            .await
        {
            debug!(logger, "Inital access procedure failed - {:?}", e);
        }
    }
}

#[async_trait]
impl<G: GnbCuCp> IndicationHandler<UlRrcMessageTransferProcedure> for F1apHandler<G> {
    async fn handle(&self, r: UlRrcMessageTransfer, logger: &Logger) {
        debug!(logger, ">> UlRrcMessageTransfer");
        let pdcp_pdu = PdcpPdu(r.rrc_container.0);

        let rrc_message_bytes = match pdcp_pdu.view_inner() {
            Ok(x) => x,
            Err(e) => {
                warn!(logger, "Invalid PDCP PDU - {:?}", e);
                return;
            }
        };

        self.rrc_handler
            .dispatch_dcch(r.gnb_cu_ue_f1ap_id.0, rrc_message_bytes, logger)
            .await;
    }
}

#[async_trait]
impl<G: GnbCuCp> EventHandler for F1apHandler<G> {
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        match event {
            TnlaEvent::Established(addr) => {
                info!(logger, "F1AP TNLA {} established with DU {}", tnla_id, addr)
            }
            TnlaEvent::Terminated => info!(logger, "F1AP TNLA {} closed", tnla_id),
        };
    }
}
