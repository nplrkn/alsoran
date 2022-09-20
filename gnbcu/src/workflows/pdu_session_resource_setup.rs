//! pdu_session_resource_setup - AMF orders setup of PDU sessions and DRBs

use anyhow::Result;
use ngap::{
    PduSessionResourceFailedToSetupListSuRes, PduSessionResourceSetupListSuRes,
    PduSessionResourceSetupRequest, PduSessionResourceSetupResponse,
};
use slog::{debug, Logger};

use crate::gnbcu_trait::Gnbcu;

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
    r: &PduSessionResourceSetupRequest,
    logger: &Logger,
) -> PduSessionResourceSetupResponse {
    debug!(&logger, "PduSessionResourceSetupRequest(Nas) << ");

    // Start by assuming all sessions are unsuccessful.
    let mut successful = vec![];
    let mut unsuccessful = r.pdu_session_resource_setup_list_su_req.0;

    // let (successful, unsuccessful) = setup_sessions(gnbcu, r, logger);

    // // Retrieve UE context by ran_ue_ngap_id.
    // debug!(&logger, "Retrieve UE {:#010x}", r.ran_ue_ngap_id.0);
    // let ue = gnbcu
    //     .retrieve(&r.ran_ue_ngap_id.0)
    //     .await
    //     .map_err(|_| Cause::RadioNetwork(CauseRadioNetwork::UnknownLocalUeNgapId))?;

    let pdu_session_resource_setup_list_su_res = if successful.is_empty() { None } else {
        Some(successful.iter().map(|x| x))
    }
    
    let pdu_session_resource_failed_to_setup_list_su_res = if unsuccessful.is_empty() { None } else {
        Some(unsuccessful.iter().map(|x| x))
    }

    PduSessionResourceSetupResponse {
        amf_ue_ngap_id: r.amf_ue_ngap_id,
        ran_ue_ngap_id: r.ran_ue_ngap_id,
        pdu_session_resource_setup_list_su_res,
        pdu_session_resource_failed_to_setup_list_su_res,
        criticality_diagnostics: None,
    }
}
