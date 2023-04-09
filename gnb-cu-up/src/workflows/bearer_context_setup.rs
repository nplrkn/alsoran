//! bearer_context_setup - creation of userplane session (supplying the UPF tunnel info)
//! and allocation of GTP tunnel at the CU-UP

use super::{GnbCuUp, Workflow};
use anyhow::Result;
use bitvec::prelude::*;
use e1ap::*;
use net::{RequestError, ResponseAction};

impl<'a, G: GnbCuUp> Workflow<'a, G> {
    pub async fn bearer_context_setup(
        &self,
        r: &BearerContextSetupRequest,
    ) -> Result<ResponseAction<BearerContextSetupResponse>, RequestError<BearerContextSetupFailure>>
    {
        self.log_message("BearerContextSetupRequest <<");

        // The userplane processing doesn't exist yet, but let's suppose it will be designed to
        // use the same GTP TEID on both sides for each bearer.
        let my_uplink_address = TransportLayerAddress(bitvec![u8,Msb0;3,3,3,3]);
        let my_downlink_address = TransportLayerAddress(bitvec![u8,Msb0;4,4,4,4]);
        let bearer_1_gtp_teid = GtpTeid(vec![1, 2, 3, 4]);

        self.log_message("BearerContextSetupResponse >>");

        Ok((
            // TODO: The following is hardcoded various things that ought to be derived from the original request
            BearerContextSetupResponse {
                gnb_cu_cp_ue_e1ap_id: r.gnb_cu_cp_ue_e1ap_id,
                gnb_cu_up_ue_e1ap_id: GnbCuUpUeE1apId(r.gnb_cu_cp_ue_e1ap_id.0),
                system_bearer_context_setup_response:
                    e1ap::SystemBearerContextSetupResponse::NgRanBearerContextSetupResponse(
                        NgRanBearerContextSetupResponse {
                            pdu_session_resource_setup_list: PduSessionResourceSetupList(vec![
                                PduSessionResourceSetupItem {
                                    pdu_session_id: PduSessionId(1),
                                    security_result: None,
                                    ng_dl_up_tnl_information: UpTnlInformation::GtpTunnel(
                                        GtpTunnel {
                                            transport_layer_address: my_uplink_address,
                                            gtp_teid: bearer_1_gtp_teid.clone(),
                                        },
                                    ),
                                    pdu_session_data_forwarding_information_response: None,
                                    ng_dl_up_unchanged: None,
                                    drb_setup_list_ng_ran: DrbSetupListNgRan(vec![
                                        DrbSetupItemNgRan {
                                            drb_id: DrbId(1),
                                            drb_data_forwarding_information_response: None,
                                            ul_up_transport_parameters: UpParameters(vec![
                                                UpParametersItem {
                                                    up_tnl_information: UpTnlInformation::GtpTunnel(
                                                        GtpTunnel {
                                                            transport_layer_address:
                                                                my_downlink_address,
                                                            gtp_teid: bearer_1_gtp_teid,
                                                        },
                                                    ),
                                                    cell_group_id: CellGroupId(1),
                                                    qos_mapping_information: None,
                                                },
                                            ]),
                                            flow_setup_list: QosFlowList(vec![QosFlowItem {
                                                qos_flow_identifier: QosFlowIdentifier(1),
                                                qos_flow_mapping_indication: None,
                                            }]),
                                            flow_failed_list: None,
                                        },
                                    ]),
                                    drb_failed_list_ng_ran: None,
                                    redundant_n_g_dl_up_tnl_information: None,
                                    redundant_pdu_session_information_used: None,
                                },
                            ]),
                            pdu_session_resource_failed_list: None,
                        },
                    ),
            },
            None,
        ))
    }
}
