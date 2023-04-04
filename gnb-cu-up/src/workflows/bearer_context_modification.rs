//! bearer_context_modification - update of userplane session, supplying the DU's tunnel info

use super::{GnbCuUp, Workflow};
use anyhow::Result;
use e1ap::*;
use net::{RequestError, ResponseAction};

impl<'a, G: GnbCuUp> Workflow<'a, G> {
    pub async fn bearer_context_modification(
        &self,
        r: &BearerContextModificationRequest,
    ) -> Result<
        ResponseAction<BearerContextModificationResponse>,
        RequestError<BearerContextModificationFailure>,
    > {
        self.log_message("BearerContextModificationRequest <<");

        // TODO - get DU GTP tunnel info

        self.log_message("BearerContextModificationResponse >>");
        Ok((
            BearerContextModificationResponse {
                gnb_cu_cp_ue_e1ap_id: r.gnb_cu_cp_ue_e1ap_id,
                gnb_cu_up_ue_e1ap_id: GnbCuUpUeE1apId(r.gnb_cu_cp_ue_e1ap_id.0),
                system_bearer_context_modification_response: None,
            },
            None,
        ))
    }
}
