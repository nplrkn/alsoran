//! pdu_session_resource_release - AMF orders release of PDU sessions and DRBs

use super::{GnbCuCp, Workflow};
use crate::datastore::UeState;
use anyhow::{bail, Result};
use asn1_per::*;
use e1ap::{
    BearerContextReleaseCommand, BearerContextReleaseProcedure, Cause as E1Cause,
    CauseRadioNetwork as E1CauseRadioNetwork, GnbCuCpUeE1apId, GnbCuUpUeE1apId,
};
use f1ap::{
    Cause as F1Cause, CauseRadioNetwork as F1CauseRadioNetwork, GnbCuUeF1apId, SrbId,
    UeContextReleaseCommand, UeContextReleaseProcedure,
};
use ngap::{
    NasPdu, PduSessionResourceReleaseCommand, PduSessionResourceReleaseResponse,
    PduSessionResourceReleaseResponseTransfer, PduSessionResourceReleasedItemRelRes,
    PduSessionResourceReleasedListRelRes,
};
use rrc::DedicatedNasMessage;
use slog::{debug, warn};

impl<'a, G: GnbCuCp> Workflow<'a, G> {
    // Pdu session resource release procedure.
    // See TS 38.401, figure 8.9.3.1-1.
    //
    // 1.    Ngap PduSessionResourceReleaseCommand(Nas) <<
    // 2a. << E1ap BearerContextReleaseCommand
    // 2b. << F1ap UeContextReleaseCommand
    // 3a. >> E1ap BearerContextReleaseComplete
    // 3b. >> F1ap UeContextReleaseComplete
    // 8.    PduSessionResourceReleaseResponse >>
    pub async fn pdu_session_resource_release(
        &self,
        r: PduSessionResourceReleaseCommand,
    ) -> PduSessionResourceReleaseResponse {
        debug!(self.logger, "PduSessionResourceReleaseCommand(Nas) << ");
        let amf_ue_ngap_id = r.amf_ue_ngap_id;
        let ran_ue_ngap_id = r.ran_ue_ngap_id;
        let pdu_session_to_release = r
            .pdu_session_resource_to_release_list_rel_cmd
            .0
            .first()
            .pdu_session_id;

        if let Err(e) = self
            .pdu_session_resource_release_inner(ran_ue_ngap_id.0, r.nas_pdu)
            .await
        {
            warn!(self.logger, "Error during session release {e}")
        }

        debug!(self.logger, "PduSessionResourceReleaseResponse >> ");
        PduSessionResourceReleaseResponse {
            amf_ue_ngap_id,
            ran_ue_ngap_id,
            criticality_diagnostics: None,
            pdu_session_resource_released_list_rel_res: PduSessionResourceReleasedListRelRes(
                nonempty![PduSessionResourceReleasedItemRelRes {
                    pdu_session_id: pdu_session_to_release,
                    pdu_session_resource_release_response_transfer:
                        PduSessionResourceReleaseResponseTransfer {
                            secondary_rat_usage_information: None,
                        }
                        .into_bytes()
                        .unwrap_or(vec![])
                }],
            ),
            user_location_information: None,
        }
    }

    pub async fn pdu_session_resource_release_inner(
        &self,
        ue_id: u32,
        nas_pdu: Option<NasPdu>,
    ) -> Result<()> {
        // Load UE.
        debug!(self.logger, "Retrieve UE {:#010x}", ue_id);
        let mut ue = self.retrieve(&ue_id).await?;
        let Some(gnb_cu_up_ue_e1ap_id) = ue.gnb_cu_up_ue_e1ap_id else {
            bail!("UE has no E1 context");
        };

        // TODO - cope with >1 PDU session being released at a time

        // Simultaneously send E1 and F1 releases.
        let e1 = self.perform_e1_bearer_release(&ue, gnb_cu_up_ue_e1ap_id);
        let f1 = self.perform_f1_context_release(&ue, nas_pdu);

        // Wait for both to complete.
        futures_lite::future::zip(e1, f1).await;

        // Update and write back UE.
        ue.gnb_cu_up_ue_e1ap_id = None;
        debug!(self.logger, "Store UE {:#010x}", ue.key);
        self.store(ue.key, ue, self.config().ue_ttl_secs).await
    }

    async fn perform_e1_bearer_release(&self, ue: &UeState, gnb_cu_up_ue_e1ap_id: GnbCuUpUeE1apId) {
        let bearer_context_release_command = BearerContextReleaseCommand {
            gnb_cu_cp_ue_e1ap_id: GnbCuCpUeE1apId(ue.key),
            gnb_cu_up_ue_e1ap_id,
            cause: E1Cause::RadioNetwork(E1CauseRadioNetwork::NormalRelease),
        };

        self.log_message("<< E1ap BearerContextReleaseCommand");
        match self
            .e1ap_request::<BearerContextReleaseProcedure>(
                bearer_context_release_command,
                self.logger,
            )
            .await
        {
            Ok(_r) => self.log_message(">> E1ap BearerContextReleaseComplete"),
            Err(e) => warn!(
                self.logger,
                "Error during E1 Bearer Context Release procedure - {e}"
            ),
        }
    }

    async fn perform_f1_context_release(&self, ue: &UeState, nas_pdu: Option<NasPdu>) {
        let rrc_container = nas_pdu.and_then(|p| {
            super::build_rrc::build_rrc_dl_information_transfer(0, DedicatedNasMessage(p.0)).ok()
        });

        let ue_context_release_command = UeContextReleaseCommand {
            gnb_cu_ue_f1ap_id: GnbCuUeF1apId(ue.key),
            gnb_du_ue_f1ap_id: ue.gnb_du_ue_f1ap_id,
            cause: F1Cause::RadioNetwork(F1CauseRadioNetwork::NormalRelease),
            rrc_container,
            srb_id: Some(SrbId(1)),
            old_gnb_du_ue_f1ap_id: None,
            execute_duplication: None,
            rrc_delivery_status_request: None,
            target_cells_to_cancel: None,
        };

        self.log_message("<< F1ap UeContextReleaseCommand");
        match self
            .f1ap_request::<UeContextReleaseProcedure>(ue_context_release_command, self.logger)
            .await
        {
            Ok(_r) => self.log_message(">> F1ap UeContextReleaseComplete"),
            Err(e) => warn!(
                self.logger,
                "Error during F1 Ue Context Release procedure - {e}"
            ),
        }
    }
}
