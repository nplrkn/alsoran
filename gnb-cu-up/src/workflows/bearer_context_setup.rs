//! bearer_context_setup - creation of userplane session (supplying the UPF tunnel info)
//! and allocation of GTP tunnel at the CU-UP

use super::{GnbCuUp, Workflow};
use crate::packet_processor::ForwardingAction;
use anyhow::{bail, Result};
use asn1_per::*;
use e1ap::*;
use slog::debug;
use xxap::*;

impl<'a, G: GnbCuUp> Workflow<'a, G> {
    pub async fn bearer_context_setup(
        &self,
        r: &BearerContextSetupRequest,
    ) -> Result<BearerContextSetupResponse> {
        self.log_message("BearerContextSetupRequest <<");

        let SystemBearerContextSetupRequest::NgRanBearerContextSetupRequest(
            NgRanBearerContextSetupRequest{pdu_session_resource_to_setup_list}) =
               &r.system_bearer_context_setup_request
        else {
            bail!("Not an NgRanBearerContextSetupRequest");
        };

        let gnb_cu_up_ue_e1ap_id = self.new_ue_ap_id();
        debug!(
            &self.logger,
            "Allocated UE context {}", gnb_cu_up_ue_e1ap_id.0
        );

        let mut setup_items = vec![];

        for to_setup_item in &pdu_session_resource_to_setup_list.0 {
            let setup_item = self
                .setup_session(gnb_cu_up_ue_e1ap_id, to_setup_item)
                .await?;
            setup_items.push(setup_item);
        }

        self.log_message("BearerContextSetupResponse >>");
        let setup_items = NonEmpty::from_vec(setup_items).unwrap();

        Ok(BearerContextSetupResponse {
            gnb_cu_cp_ue_e1ap_id: r.gnb_cu_cp_ue_e1ap_id,
            gnb_cu_up_ue_e1ap_id,
            system_bearer_context_setup_response:
                e1ap::SystemBearerContextSetupResponse::NgRanBearerContextSetupResponse(
                    NgRanBearerContextSetupResponse {
                        pdu_session_resource_setup_list: PduSessionResourceSetupList(setup_items),
                        pdu_session_resource_failed_list: None,
                    },
                ),
        })
    }

    async fn setup_session(
        &self,
        ue_id: GnbCuUpUeE1apId,
        setup_item: &PduSessionResourceToSetupItem,
    ) -> Result<PduSessionResourceSetupItem> {
        // Create the uplink forwarding rule.
        let session_1_uplink_gtp_teid = self.create_uplink_teid(ue_id.0, 1);
        let forwarding_action = create_forwarding_action(setup_item);

        // Install it in the packet processor.
        self.set_uplink_forwarding_action(session_1_uplink_gtp_teid.clone(), forwarding_action)
            .await;

        // We also need to supply our downlink TEID now, even though we program a rule for this
        // until we learn about the DU's TEID at the later modification stage.
        let session_1_downlink_gtp_teid = self.create_downlink_teid(ue_id.0, 1);

        // Form response

        // The CU-UP just supports a single IP address, so the uplink and downlink IPs are the same.
        let my_n3_address: TransportLayerAddress = self.config().userplane_ip_address.into();
        let my_f1u_address = my_n3_address.clone();

        Ok(PduSessionResourceSetupItem {
            pdu_session_id: PduSessionId(1),
            security_result: None,
            ng_dl_up_tnl_information: UpTnlInformation::GtpTunnel(GtpTunnel {
                transport_layer_address: my_n3_address,
                gtp_teid: session_1_downlink_gtp_teid,
            }),
            pdu_session_data_forwarding_information_response: None,
            ng_dl_up_unchanged: None,
            drb_setup_list_ng_ran: DrbSetupListNgRan(nonempty![DrbSetupItemNgRan {
                drb_id: DrbId(1),
                drb_data_forwarding_information_response: None,
                ul_up_transport_parameters: UpParameters(nonempty![UpParametersItem {
                    up_tnl_information: UpTnlInformation::GtpTunnel(GtpTunnel {
                        transport_layer_address: my_f1u_address,
                        gtp_teid: session_1_uplink_gtp_teid,
                    }),
                    cell_group_id: CellGroupId(1),
                    qos_mapping_information: None,
                }]),
                flow_setup_list: QosFlowList(nonempty![QosFlowItem {
                    qos_flow_identifier: QosFlowIdentifier(1),
                    qos_flow_mapping_indication: None,
                }]),
                flow_failed_list: None,
            }]),
            drb_failed_list_ng_ran: None,
            redundant_n_g_dl_up_tnl_information: None,
            redundant_pdu_session_information_used: None,
        })
    }
}

fn create_forwarding_action(setup_item: &PduSessionResourceToSetupItem) -> ForwardingAction {
    let UpTnlInformation::GtpTunnel(gtp_tunnel) = &setup_item.ng_ul_up_tnl_information;
    ForwardingAction {
        remote_tunnel_info: gtp_tunnel.clone(),
    }
}
