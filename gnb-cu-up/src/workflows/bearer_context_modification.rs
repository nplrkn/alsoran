//! bearer_context_modification - update of userplane session, supplying the DU's tunnel info

use super::{GnbCuUp, Workflow};
use anyhow::Result;
use asn1_per::*;
use e1ap::*;
use net::{RequestError, ResponseAction};
use xxap::PduSessionId;

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
                system_bearer_context_modification_response: Some(
                    SystemBearerContextModificationResponse::NgRanBearerContextModificationResponse(
                        NgRanBearerContextModificationResponse {
                            pdu_session_resource_setup_mod_list: None,
                            pdu_session_resource_failed_mod_list: None,
                            pdu_session_resource_modified_list: Some(
                                PduSessionResourceModifiedList(nonempty![
                                    PduSessionResourceModifiedItem {
                                        pdu_session_id: PduSessionId(1), // TODO
                                        ng_dl_up_tnl_information: None,
                                        security_result: None,
                                        pdu_session_data_forwarding_information_response: None,
                                        drb_setup_list_ng_ran: None,
                                        drb_failed_list_ng_ran: None,
                                        drb_modified_list_ng_ran: None,
                                        drb_failed_to_modify_list_ng_ran: None,
                                        redundant_n_g_dl_up_tnl_information: None,
                                    },
                                ]),
                            ),
                            pdu_session_resource_failed_to_modify_list: None,
                            retainability_measurements_info: None,
                        },
                    ),
                ),
            },
            None,
        ))
    }
}
