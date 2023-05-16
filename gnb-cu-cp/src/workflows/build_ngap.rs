//! build_ngap - construction of NGAP messages

use crate::gnb_cu_cp::GnbCuCp;
use anyhow::Result;
use asn1_per::SerDes;
use bitvec::prelude::*;
use ngap::{
    AssociatedQosFlowItem, AssociatedQosFlowList, GlobalGnbId, GlobalRanNodeId, GnbId,
    PduSessionResourceSetupItemSuRes, PduSessionResourceSetupResponseTransfer, PlmnIdentity,
    QosFlowPerTnlInformation, UpTransportLayerInformation,
};
use xxap::{GtpTunnel, PduSessionId};

pub fn build_global_ran_node_id<T: GnbCuCp>(gnb_cu_cp: &T) -> GlobalRanNodeId {
    GlobalRanNodeId::GlobalGnbId(GlobalGnbId {
        plmn_identity: PlmnIdentity(gnb_cu_cp.config().plmn.clone()),
        gnb_id: GnbId::GnbId(bitvec![u8,Msb0; 1; 22]),
    })
}

pub fn build_pdu_session_resource_setup_item_su_res(
    pdu_session_id: PduSessionId,
    gtp_tunnel: GtpTunnel,
) -> Result<PduSessionResourceSetupItemSuRes> {
    Ok(PduSessionResourceSetupItemSuRes {
        pdu_session_id,
        pdu_session_resource_setup_response_transfer: PduSessionResourceSetupResponseTransfer {
            dl_qos_flow_per_tnl_information: QosFlowPerTnlInformation {
                up_transport_layer_information: UpTransportLayerInformation::GtpTunnel(gtp_tunnel),
                associated_qos_flow_list: AssociatedQosFlowList(vec![AssociatedQosFlowItem {
                    qos_flow_identifier: ngap::QosFlowIdentifier(1),
                    qos_flow_mapping_indication: None,
                    current_qos_para_set_index: None,
                }]),
            },
            additional_dl_qos_flow_per_tnl_information: None,
            security_result: None,
            qos_flow_failed_to_setup_list: None,
            redundant_dl_qos_flow_per_tnl_information: None,
            additional_redundant_dl_qos_flow_per_tnl_information: None,
            used_rsn_information: None,
            global_ran_node_id: None,
        }
        .into_bytes()?,
    })
}
