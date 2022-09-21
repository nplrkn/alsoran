//! pdu_session_resource_setup - AMF orders setup of PDU sessions and DRBs

use anyhow::Result;
use ngap::{
    AmfUeNgapId, PduSessionResourceFailedToSetupItemSuRes,
    PduSessionResourceFailedToSetupListSuRes, PduSessionResourceSetupItemSuReq,
    PduSessionResourceSetupItemSuRes, PduSessionResourceSetupListSuRes,
    PduSessionResourceSetupRequest, PduSessionResourceSetupResponse, RanUeNgapId,
};
use slog::{debug, Logger};

use crate::{datastore::UeState, gnbcu_trait::Gnbcu};

// Pdu session resource setup procedure.
//
// See documentation/session establishment.md
//
// 1.    Ngap PduSessionResourceSetupRequest(Nas) <<
// 2. << E1ap BearerContextSetup
// 3. >> E1ap BearerContextSetupResponse
// 4. << F1ap UeContextSetupRequest
// 5. >> F1ap UeContextSetupResponse
// 6. << Dl Rrc Message Transfer + Rrc Reconfiguration + Nas PDU Session Establishment Accept
// 7. >> Ul Rrc Message Transfer + Rrc Reconfiguration Complete
// 8.    Pdu Session Resource Setup Response >>

pub async fn pdu_session_resource_setup<G: Gnbcu>(
    gnbcu: &G,
    r: PduSessionResourceSetupRequest,
    logger: &Logger,
) -> PduSessionResourceSetupResponse {
    debug!(&logger, "PduSessionResourceSetupRequest(Nas) << ");

    let mut successful = Vec::<PduSessionResourceSetupItemSuReq>::new();
    let mut unsuccessful = Vec::<PduSessionResourceSetupItemSuReq>::new();

    // Retrieve UE context by ran_ue_ngap_id.
    debug!(&logger, "Retrieve UE {:#010x}", r.ran_ue_ngap_id.0);
    match gnbcu.retrieve(&r.ran_ue_ngap_id.0).await {
        Ok(mut ue) => {
            for session in r.pdu_session_resource_setup_list_su_req.0.into_iter() {
                match setup_session(gnbcu, &mut ue, &session, logger).await {
                    Ok(_) => successful.push(session),
                    Err(_) => unsuccessful.push(session),
                }
            }
            // write UE
        }
        Err(e) => {
            debug!(logger, "Failed to retrieve UE context - {:?}", e);
            unsuccessful = r.pdu_session_resource_setup_list_su_req.0;
        }
    }

    let pdu_session_resource_setup_list_su_res = if successful.is_empty() {
        None
    } else {
        Some(PduSessionResourceSetupListSuRes(
            successful
                .into_iter()
                .map(|x| PduSessionResourceSetupItemSuRes {
                    pdu_session_id: x.pdu_session_id,
                    pdu_session_resource_setup_response_transfer: x
                        .pdu_session_resource_setup_request_transfer,
                })
                .collect(),
        ))
    };

    let pdu_session_resource_failed_to_setup_list_su_res = if unsuccessful.is_empty() {
        None
    } else {
        Some(PduSessionResourceFailedToSetupListSuRes(
            unsuccessful
                .into_iter()
                .map(|x| PduSessionResourceFailedToSetupItemSuRes {
                    pdu_session_id: x.pdu_session_id,
                    pdu_session_resource_setup_unsuccessful_transfer: x
                        .pdu_session_resource_setup_request_transfer,
                })
                .collect(),
        ))
    };

    PduSessionResourceSetupResponse {
        amf_ue_ngap_id: AmfUeNgapId(r.amf_ue_ngap_id.0), // TODO: type should be Copy
        ran_ue_ngap_id: RanUeNgapId(r.ran_ue_ngap_id.0), // TODO: type should be Copy
        pdu_session_resource_setup_list_su_res,
        pdu_session_resource_failed_to_setup_list_su_res,
        criticality_diagnostics: None,
    }
}

pub async fn setup_session<G: Gnbcu>(
    gnbcu: &G,
    ue: &mut UeState,
    r: &PduSessionResourceSetupItemSuReq,
    logger: &Logger,
) -> Result<()> {
    todo!()
}
