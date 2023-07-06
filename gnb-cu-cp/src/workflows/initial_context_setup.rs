//! initial_context_setup - in which the secure signaling channel is established between UE and 5G core through the GNB

use super::{GnbCuCp, Workflow};
use anyhow::Result;
use f1ap::SrbId;
use net::ResponseAction;
use ngap::*;
use rrc::*;
use slog::debug;

impl<'a, G: GnbCuCp> Workflow<'a, G> {
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
    pub async fn initial_context_setup(
        &self,
        r: &InitialContextSetupRequest,
    ) -> Result<ResponseAction<InitialContextSetupResponse>, Cause> {
        self.log_message("InitialContextSetupRequest(Nas) << ");

        let ue_key = r.ran_ue_ngap_id.0;

        // Retrieve UE context by ran_ue_ngap_id.
        debug!(self.logger, "Retrieve UE {:#010x}", ue_key);
        let ue = self
            .retrieve(&ue_key)
            .await
            .map_err(|_| Cause::RadioNetwork(CauseRadioNetwork::UnknownLocalUeNgapId))?;

        // Build Security Mode command and wrap it in an RrcContainer.
        let rrc_transaction = self.new_rrc_transaction(&ue).await;
        let rrc_container = super::build_rrc::build_rrc_security_mode_command(0)
            .map_err(|_| Cause::Misc(CauseMisc::Unspecified))?;

        if let Some(_sessions) = &r.pdu_session_resource_setup_list_cxt_req {
            // --- Sessions needed ---
            // TODO
            self.log_message_error("Combined context and session setup not suppported");
            return Err(Cause::Misc(CauseMisc::Unspecified));
        } else {
            // --- No sessions needed ---
            self.log_message("<< SecurityModeCommand");
            self.send_rrc_to_ue(&ue, SrbId(1), rrc_container, self.logger)
                .await;
        };

        // Receive Security Mode Complete.
        let _rrc_security_mode_complete = rrc_transaction
            .recv()
            .await
            .map_err(|_| Cause::Misc(CauseMisc::Unspecified))?;
        self.log_message(">> SecurityModeComplete");

        if let Some(nas) = r.nas_pdu.clone() {
            if let Err(e) = self.send_nas_to_ue(&ue, DedicatedNasMessage(nas.0)).await {
                debug!(self.logger, "Failed to send NAS to UE- {:?}", e)
            }
        } else {
            debug!(
                self.logger,
                "No Nas and no sessions on initial context create"
            );
        }

        // Write back UE.
        debug!(self.logger, "Store UE {:#010x}", ue_key);
        if let Err(e) = self.store(ue_key, ue, self.config().ue_ttl_secs).await {
            debug!(self.logger, "Failed to write back UE- {:?}", e)
        }

        // Reply to the AMF.
        self.log_message("InitialContextSetupResponse >>");
        Ok((
            InitialContextSetupResponse {
                amf_ue_ngap_id: r.amf_ue_ngap_id,
                ran_ue_ngap_id: RanUeNgapId(ue_key),
                pdu_session_resource_setup_list_cxt_res: None,
                pdu_session_resource_failed_to_setup_list_cxt_res: None,
                criticality_diagnostics: None,
            },
            None,
        ))
    }
}
