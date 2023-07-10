//! e1ap - E1AP entry points into the GNB-CU

use crate::workflows::Workflow;

use crate::GnbCuUp;
use async_trait::async_trait;
use e1ap::*;
use net::{EventHandler, RequestError, RequestProvider, ResponseAction, TnlaEvent};
use slog::{info, warn, Logger};

#[derive(Clone)]
pub struct E1apHandler<G: GnbCuUp> {
    gnb_cu_up: G,
}

impl<G: GnbCuUp> E1apHandler<G> {
    pub fn new_e1ap_application(gnb_cu_up: G) -> E1apUp<E1apHandler<G>> {
        E1apUp::new(E1apHandler { gnb_cu_up })
    }
}

#[async_trait]
impl<G: GnbCuUp> EventHandler for E1apHandler<G> {
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        match event {
            TnlaEvent::Established(addr) => {
                info!(
                    logger,
                    "E1AP TNLA {} established with CU-CP {}", tnla_id, addr
                )
            }
            TnlaEvent::Terminated => info!(logger, "E1AP TNLA {} closed", tnla_id),
        };
    }
}

#[async_trait]
impl<G: GnbCuUp> RequestProvider<BearerContextSetupProcedure> for E1apHandler<G> {
    async fn request(
        &self,
        r: BearerContextSetupRequest,
        logger: &Logger,
    ) -> Result<ResponseAction<BearerContextSetupResponse>, RequestError<BearerContextSetupFailure>>
    {
        Workflow::new(&self.gnb_cu_up, logger)
            .bearer_context_setup(&r)
            .await
            .map(|ok_response| (ok_response, None))
            .map_err(|e| {
                warn!(logger, "Failed bearer context setup - {e}");
                RequestError::UnsuccessfulOutcome(BearerContextSetupFailure {
                    gnb_cu_cp_ue_e1ap_id: r.gnb_cu_cp_ue_e1ap_id,
                    gnb_cu_up_ue_e1ap_id: None,
                    cause: Cause::RadioNetwork(CauseRadioNetwork::Unspecified),
                    criticality_diagnostics: None,
                })
            })
    }
}

#[async_trait]
impl<G: GnbCuUp> RequestProvider<BearerContextModificationProcedure> for E1apHandler<G> {
    async fn request(
        &self,
        r: BearerContextModificationRequest,
        logger: &Logger,
    ) -> Result<
        ResponseAction<BearerContextModificationResponse>,
        RequestError<BearerContextModificationFailure>,
    > {
        let gnb_cu_cp_ue_e1ap_id = r.gnb_cu_cp_ue_e1ap_id;
        let gnb_cu_up_ue_e1ap_id = r.gnb_cu_up_ue_e1ap_id;

        Workflow::new(&self.gnb_cu_up, logger)
            .bearer_context_modification(r)
            .await
            .map(|ok_response| (ok_response, None))
            .map_err(|e| {
                warn!(logger, "Failed bearer context modifification - {e}");
                RequestError::UnsuccessfulOutcome(BearerContextModificationFailure {
                    gnb_cu_cp_ue_e1ap_id,
                    gnb_cu_up_ue_e1ap_id,
                    cause: Cause::RadioNetwork(CauseRadioNetwork::Unspecified),
                    criticality_diagnostics: None,
                })
            })
    }
}

#[async_trait]
impl<G: GnbCuUp> RequestProvider<BearerContextReleaseProcedure> for E1apHandler<G> {
    async fn request(
        &self,
        r: BearerContextReleaseCommand,
        logger: &Logger,
    ) -> Result<ResponseAction<BearerContextReleaseComplete>, RequestError<()>> {
        Ok((
            Workflow::new(&self.gnb_cu_up, logger)
                .bearer_context_release(&r)
                .await,
            None,
        ))
    }
}

#[async_trait]
impl<G: GnbCuUp> RequestProvider<GnbCuCpConfigurationUpdateProcedure> for E1apHandler<G> {
    async fn request(
        &self,
        r: GnbCuCpConfigurationUpdate,
        logger: &Logger,
    ) -> Result<
        ResponseAction<GnbCuCpConfigurationUpdateAcknowledge>,
        RequestError<GnbCuCpConfigurationUpdateFailure>,
    > {
        Workflow::new(&self.gnb_cu_up, logger)
            .gnb_cu_cp_configuration_update(r)
            .await
            .map(|ok_response| (ok_response, None))
            .map_err(RequestError::UnsuccessfulOutcome)
    }
}
