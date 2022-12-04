//! ngap - NGAP entry points into the GNB-CU

use super::GnbCuCp;
use crate::workflows::Workflow;
use anyhow::Result;
use async_trait::async_trait;
use net::{EventHandler, IndicationHandler, RequestError, RequestProvider, TnlaEvent};
use ngap::*;
use slog::{debug, info, Logger};

impl<G: GnbCuCp> RequestProvider<NgSetupProcedure> for NgapHandler<G> {}

#[derive(Clone)]
pub struct NgapHandler<G> {
    gnb_cu_cp: G,
}

impl<G: GnbCuCp> NgapHandler<G> {
    pub fn new_ngap_application(gnb_cu_cp: G) -> NgapGnb<NgapHandler<G>> {
        NgapGnb::new(NgapHandler { gnb_cu_cp })
    }
}
#[async_trait]
impl<G: GnbCuCp> EventHandler for NgapHandler<G> {
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        match event {
            TnlaEvent::Established(addr) => {
                info!(logger, "NGAP TNLA {} established to {}", tnla_id, addr);
            }
            TnlaEvent::Terminated => info!(logger, "NGAP TNLA {} closed", tnla_id),
        };
        // TODO
    }
}

#[async_trait]
impl<G: GnbCuCp> IndicationHandler<DownlinkNasTransportProcedure> for NgapHandler<G> {
    async fn handle(&self, i: DownlinkNasTransport, logger: &Logger) {
        if let Err(e) = Workflow::new(&self.gnb_cu_cp, logger).downlink_nas(i).await {
            debug!(logger, "Downlink Nas Trasnport procedure failed - {:?}", e);
        };
    }
}

#[async_trait]
impl<G: GnbCuCp> RequestProvider<InitialContextSetupProcedure> for NgapHandler<G> {
    async fn request(
        &self,
        r: InitialContextSetupRequest,
        logger: &Logger,
    ) -> Result<InitialContextSetupResponse, RequestError<InitialContextSetupFailure>> {
        Workflow::new(&self.gnb_cu_cp, logger)
            .initial_context_setup(&r)
            .await
            .map_err(|cause| {
                RequestError::UnsuccessfulOutcome(InitialContextSetupFailure {
                    amf_ue_ngap_id: r.amf_ue_ngap_id,
                    ran_ue_ngap_id: r.ran_ue_ngap_id,
                    pdu_session_resource_failed_to_setup_list_cxt_fail: None,
                    cause,
                    criticality_diagnostics: None,
                })
            })
    }
}

#[async_trait]
impl<G: GnbCuCp> IndicationHandler<AmfStatusIndicationProcedure> for NgapHandler<G> {
    async fn handle(&self, i: AmfStatusIndication, logger: &Logger) {
        Workflow::new(&self.gnb_cu_cp, logger)
            .amf_status_indication(i)
            .await;
    }
}

#[async_trait]
impl<G: GnbCuCp> RequestProvider<PduSessionResourceSetupProcedure> for NgapHandler<G> {
    async fn request(
        &self,
        r: PduSessionResourceSetupRequest,
        logger: &Logger,
    ) -> Result<PduSessionResourceSetupResponse, RequestError<()>> {
        Ok(Workflow::new(&self.gnb_cu_cp, logger)
            .pdu_session_resource_setup(r)
            .await)
    }
}
