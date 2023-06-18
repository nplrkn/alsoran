//! bearer_context_modification - update of userplane session, supplying the DU's tunnel info

use super::{GnbCuUp, Workflow};
use asn1_per::*;
use crate::packet_processor::ForwardingAction;
use anyhow::{bail, Result, ensure};
use e1ap::*;
use slog::debug;
use xxap::PduSessionId;

impl<'a, G: GnbCuUp> Workflow<'a, G> {
    pub async fn bearer_context_modification(
        &self,
        r: BearerContextModificationRequest,
    ) -> Result<BearerContextModificationResponse> {
        self.log_message("BearerContextModificationRequest <<");

        // May be later replaced with a retrieve
        debug!(&self.logger, "Modification of UE context {}",r.gnb_cu_up_ue_e1ap_id.0);
        ensure!(self.bearer_context_exists(r.gnb_cu_up_ue_e1ap_id.0));

        let Some(SystemBearerContextModificationRequest::NgRanBearerContextModificationRequest(
            NgRanBearerContextModificationRequest{ 
            pdu_session_resource_to_modify_list: Some(pdu_session_resource_to_modify_list), 
            ..
            })) = r.system_bearer_context_modification_request 
        else {
            bail!("Not an NgRanBearerContextModificationRequestRequest");
        };

        let mut mod_items = vec![];
        let mut to_mod_items: Vec<PduSessionResourceToModifyItem> = pdu_session_resource_to_modify_list.0.into();
        for to_mod_item in to_mod_items.drain(..) {
            let mod_item = self.modify_session(r.gnb_cu_up_ue_e1ap_id, to_mod_item).await?;
            mod_items.push(mod_item);
        }
        let pdu_session_resource_modified_list = NonEmpty::from_vec(mod_items).map(PduSessionResourceModifiedList);

        self.log_message("BearerContextModificationResponse >>");
        Ok(BearerContextModificationResponse {
            gnb_cu_cp_ue_e1ap_id: r.gnb_cu_cp_ue_e1ap_id,
            gnb_cu_up_ue_e1ap_id: GnbCuUpUeE1apId(r.gnb_cu_cp_ue_e1ap_id.0),
            system_bearer_context_modification_response: Some(
                SystemBearerContextModificationResponse::NgRanBearerContextModificationResponse(
                    NgRanBearerContextModificationResponse {
                        pdu_session_resource_setup_mod_list: None,
                        pdu_session_resource_failed_mod_list: None,
                        pdu_session_resource_modified_list,
                        pdu_session_resource_failed_to_modify_list: None,
                        retainability_measurements_info: None,
                    },
                ),
            ),
        })
    }

    async fn modify_session(
        &self,
        ue_id: GnbCuUpUeE1apId,
        mod_item: PduSessionResourceToModifyItem,
    ) -> Result<PduSessionResourceModifiedItem> {
        // TODO: support > 1 session.

        // We have already signalled a downlink GTP TEID back to the GNB-CU-CP at setup time.
        // Now we have enough info need to program a forwarding action for it.  Our GTP TEIDs are
        // encoded deterministically from other info so we can simply reconsistute it here.
        let session_1_downlink_gtp_teid = self.create_downlink_teid(ue_id.0, 1);

        let Some(modify_list) = mod_item.drb_to_modify_list_ng_ran else {
            bail!("No modify list on PduSessionResourceToModifyItem")
        };

        let Some(up_parameters) = modify_list.0.head.dl_up_parameters else {
            bail!("No UP parameters on DrbToModifyItemNgRan")
        };

        let UpTnlInformation::GtpTunnel(remote_tunnel_info) = up_parameters.0.head.up_tnl_information;

        let forwarding_action = ForwardingAction {
            remote_tunnel_info,
        };

        // Install it in the packet processor.
        self.install_forwarding_rule(session_1_downlink_gtp_teid, forwarding_action).await;

        Ok(PduSessionResourceModifiedItem {
            pdu_session_id: PduSessionId(1), // TODO
            ng_dl_up_tnl_information: None,
            security_result: None,
            pdu_session_data_forwarding_information_response: None,
            drb_setup_list_ng_ran: None,
            drb_failed_list_ng_ran: None,
            drb_modified_list_ng_ran: None,
            drb_failed_to_modify_list_ng_ran: None,
            redundant_n_g_dl_up_tnl_information: None,
        })
    }
}
