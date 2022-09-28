//! initial_context_setup - in which the secure signaling channel is established between UE and 5G core through the GNB

use super::Gnbcu;
use anyhow::Result;
use f1ap::{SrbId, UeContextSetupProcedure};
use ngap::*;
use rrc::*;
use slog::{debug, Logger};

// Initial context setup procedure.
// 1.    Ngap InitialContextSetupRequest(maybe Nas) <<
// --- CONDITIONAL IF PDUS ARE PRESENT -----
// 2. << F1ap UeContextSetup(Rrc SecurityModeCommand)
// 3. >> F1ap Ue Context Setup Response
// -----------------------------------------
// 4. >> Rrc SecurityModeComplete
// --- CONDITIONAL IF PDUS ARE PRESENT -----
// 5. << Rrc RrcReconfiguration(maybe Nas)
// 6. >> Rrc RrcReconfigurationComplete
// ---- NO PDUS ---
// -----------------------------------------
// 7.    Ngap InitialContextSetupResponse >>
pub async fn initial_context_setup<G: Gnbcu>(
    gnbcu: &G,
    r: &InitialContextSetupRequest,
    logger: &Logger,
) -> Result<InitialContextSetupResponse, Cause> {
    debug!(&logger, "InitialContextSetupRequest(Nas) << ");

    // Retrieve UE context by ran_ue_ngap_id.
    debug!(&logger, "Retrieve UE {:#010x}", r.ran_ue_ngap_id.0);
    let ue = gnbcu
        .retrieve(&r.ran_ue_ngap_id.0)
        .await
        .map_err(|_| Cause::RadioNetwork(CauseRadioNetwork::UnknownLocalUeNgapId))?;

    // Build Security Mode command and wrap it in an RrcContainer.
    let rrc_transaction = gnbcu.new_rrc_transaction(&ue).await;
    let rrc_container = super::build_rrc::build_rrc_security_mode_command(2)
        .map_err(|_| Cause::Misc(CauseMisc::Unspecified))?;

    if let Some(_sessions) = &r.pdu_session_resource_setup_list_cxt_req {
        // --- Sessions needed ---
        // TODO: implementation incomplete and this arm not tested

        // Build Ue Context Setup request and include the Rrc security mode command.
        let ue_context_setup_request =
            super::build_f1ap::build_ue_context_setup_request(gnbcu, &r, &ue, Some(rrc_container));

        // Send to GNB-DU and get back the response to the (outer) UE Context Setup.
        debug!(&logger, "<< UeContextSetup(SecurityModeCommand)");
        let _ue_context_setup_response = gnbcu
            .f1ap_request::<UeContextSetupProcedure>(ue_context_setup_request, &logger)
            .await
            .map_err(|_| Cause::RadioNetwork(CauseRadioNetwork::Unspecified))?;
        debug!(&logger, ">> UeContextSetupResponse");
    } else {
        // --- No sessions needed ---
        debug!(&logger, "<< SecurityModeCommand");
        gnbcu
            .send_rrc_to_ue(&ue, SrbId(1), rrc_container, logger)
            .await;
    }

    // Receive Security Mode Complete.
    let _rrc_security_mode_complete = rrc_transaction
        .recv()
        .await
        .map_err(|_| Cause::Misc(CauseMisc::Unspecified))?;
    debug!(&logger, ">> SecurityModeComplete");

    if let Some(_sessions) = &r.pdu_session_resource_setup_list_cxt_req {
        // --- Sessions needed ---
        // TODO: implementation incomplete and this arm not tested

        // Perform Rrc Reconfiguration including the Nas message from earlier.
        let rrc_transaction = gnbcu.new_rrc_transaction(&ue).await;
        let rrc_container =
            super::build_rrc::build_rrc_reconfiguration(3, r.nas_pdu.clone().map(|x| vec![x.0]))
                .map_err(|_| Cause::Misc(CauseMisc::Unspecified))?;

        // Send to the UE and get back the response.
        debug!(&logger, "<< RrcReconfiguration");
        gnbcu
            .send_rrc_to_ue(&ue, SrbId(1), rrc_container, logger)
            .await;
        let _rrc_reconfiguration_complete: UlDcchMessage = rrc_transaction
            .recv()
            .await
            .map_err(|_| Cause::Misc(CauseMisc::Unspecified))?;
        debug!(&logger, ">> RrcReconfigurationComplete");
    } else if let Some(nas) = r.nas_pdu.clone() {
        if let Err(e) =
            super::downlink_nas::send_nas_to_ue(gnbcu, &ue, DedicatedNasMessage(nas.0), logger)
                .await
        {
            debug!(&logger, "Failed to send NAS to UE- {:?}", e)
        }
    } else {
        debug!(&logger, "No Nas and no sessions on initial context create");
    }

    // Reply to the AMF.
    debug!(&logger, "InitialContextSetupResponse >>");
    Ok(InitialContextSetupResponse {
        amf_ue_ngap_id: r.amf_ue_ngap_id.clone(),
        ran_ue_ngap_id: RanUeNgapId(ue.key),
        pdu_session_resource_setup_list_cxt_res: None,
        pdu_session_resource_failed_to_setup_list_cxt_res: None,
        criticality_diagnostics: None,
    })
}
