//! bearer_context_modification - update of userplane session, supplying the DU's tunnel info

use super::{GnbCuUp, Workflow};
use anyhow::Result;
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
            pdu_session_resource_to_modify_list: Some(mut pdu_session_resource_to_modify_list), 
            ..
            })) = r.system_bearer_context_modification_request 
        else {
            bail!("Not an NgRanBearerContextModificationRequestRequest");
        };

        let mut mod_items = vec![];
        for to_mod_item in pdu_session_resource_to_modify_list.0.drain(..) {
            let mod_item = self.modify_session(r.gnb_cu_up_ue_e1ap_id, to_mod_item).await?;
            mod_items.push(mod_item);
        }

        self.log_message("BearerContextModificationResponse >>");
        Ok(BearerContextModificationResponse {
            gnb_cu_cp_ue_e1ap_id: r.gnb_cu_cp_ue_e1ap_id,
            gnb_cu_up_ue_e1ap_id: GnbCuUpUeE1apId(r.gnb_cu_cp_ue_e1ap_id.0),
            system_bearer_context_modification_response: Some(
                SystemBearerContextModificationResponse::NgRanBearerContextModificationResponse(
                    NgRanBearerContextModificationResponse {
                        pdu_session_resource_setup_mod_list: None,
                        pdu_session_resource_failed_mod_list: None,
                        pdu_session_resource_modified_list: Some(PduSessionResourceModifiedList(
                            mod_items,
                        )),
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

        // Get the remote tunnel information of the DU.
        let Some(UpTnlInformation::GtpTunnel(first_drb_tunnel)) = &mod_item.drb_to_modify_list_ng_ran
            .map(|mut list| list.0.pop())
            .flatten()
            .map(|drb_item| drb_item.dl_up_parameters)
            .flatten()
            .map(|mut up_parameters| up_parameters.0.pop())
            .flatten()
            .map(|p| p.up_tnl_information)
         else {
            bail!("Missing downlink UpParameters on PduSessionResourceModifiedItem")
        };

        let forwarding_action = ForwardingAction {
            remote_tunnel_info: first_drb_tunnel.clone(),
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
