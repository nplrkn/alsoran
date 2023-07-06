//! bearer_context_release - release of userplane session at the CU-UP

use super::{GnbCuUp, Workflow};
use e1ap::*;

impl<'a, G: GnbCuUp> Workflow<'a, G> {
    pub async fn bearer_context_release(
        &self,
        r: &BearerContextReleaseCommand,
    ) -> BearerContextReleaseComplete {
        self.log_message("BearerContextReleaseCommand <<");

        self.delete_bearer_context(r.gnb_cu_up_ue_e1ap_id.0).await;

        self.log_message(">> BearerContextReleaseComplete");
        BearerContextReleaseComplete {
            gnb_cu_cp_ue_e1ap_id: r.gnb_cu_cp_ue_e1ap_id,
            gnb_cu_up_ue_e1ap_id: r.gnb_cu_up_ue_e1ap_id,
            criticality_diagnostics: None,
            retainability_measurements_info: None,
        }
    }
}
