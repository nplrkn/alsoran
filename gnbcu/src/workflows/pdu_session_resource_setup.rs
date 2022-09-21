//! pdu_session_resource_setup - AMF orders setup of PDU sessions and DRBs

use anyhow::Result;
use bitvec::prelude::*;
use e1ap::*;
use net::AperSerde;
use ngap::{
    AmfUeNgapId, PduSessionResourceFailedToSetupItemSuRes,
    PduSessionResourceFailedToSetupListSuRes, PduSessionResourceSetupItemSuReq,
    PduSessionResourceSetupItemSuRes, PduSessionResourceSetupListSuRes,
    PduSessionResourceSetupRequest, PduSessionResourceSetupRequestTransfer,
    PduSessionResourceSetupResponse, RanUeNgapId,
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
    let session_params = PduSessionResourceSetupRequestTransfer::from_bytes(
        &r.pdu_session_resource_setup_request_transfer,
    )?;
    let ue_dl_aggregate_maximum_bit_rate = BitRate(0);
    let item = PduSessionResourceToSetupItem {
        pdu_session_id: PduSessionId(r.pdu_session_id.0),
        pdu_session_type: PduSessionType::Ipv4,
        snssai: Snssai {
            sst: r.s_nssai.sst.0.clone(),
            sd: r.s_nssai.sd.clone().map(|x| x.0),
        },
        security_indication: SecurityIndication {
            integrity_protection_indication: IntegrityProtectionIndication::Preferred,
            confidentiality_protection_indication: ConfidentialityProtectionIndication::Preferred,
            maximum_i_pdatarate: None,
        },
        pdu_session_resource_dl_ambr: None,
        // TODO: frunk convert
        ng_ul_up_tnl_information: UpTnlInformation::GtpTunnel(GtpTunnel {
            transport_layer_address: TransportLayerAddress(bitvec![u8,Msb0;0,1,1,0]),
            gtp_teid: GtpTeid(vec![1, 2, 3, 4]),
        }),
        pdu_session_data_forwarding_information_request: None,
        pdu_session_inactivity_timer: None,
        existing_allocated_ng_dl_up_tnl_info: None,
        network_instance: None,
        drb_to_setup_list_ng_ran: DrbToSetupListNgRan(vec![DrbToSetupItemNgRan {
            drb_id: DrbId(1),
            sdap_configuration: SdapConfiguration {
                default_drb: DefaultDrb::True,
                sdap_header_ul: SdapHeaderUl::Present,
                sdap_header_dl: SdapHeaderDl::Present,
            },
            pdcp_configuration: PdcpConfiguration {
                pdcp_sn_size_ul: PdcpSnSize::S12,
                pdcp_sn_size_dl: PdcpSnSize::S12,
                rlc_mode: RlcMode::RlcTm,
                rohc_parameters: None,
                t_reordering_timer: None,
                discard_timer: None,
                ul_data_split_threshold: None,
                pdcp_duplication: None,
                pdcp_reestablishment: None,
                pdcp_data_recovery: None,
                duplication_activation: None,
                out_of_order_delivery: None,
            },
            cell_group_information: CellGroupInformation(vec![CellGroupInformationItem {
                cell_group_id: CellGroupId(1),
                ul_configuration: None,
                dl_tx_stop: None,
                rat_type: None,
            }]),
            qos_flow_information_to_be_setup: QosFlowQosParameterList(vec![
                QosFlowQosParameterItem {
                    qos_flow_identifier: QosFlowIdentifier(1),
                    qos_flow_level_qos_parameters: QosFlowLevelQosParameters {
                        qos_characteristics: QosCharacteristics::NonDynamic5qi(
                            NonDynamic5qiDescriptor {
                                five_qi: 1,
                                qos_priority_level: None,
                                averaging_window: None,
                                max_data_burst_volume: None,
                            },
                        ),
                        ngran_allocation_retention_priority: NgranAllocationAndRetentionPriority {
                            priority_level: PriorityLevel(1),
                            pre_emption_capability: PreEmptionCapability::MayTriggerPreEmption,
                            pre_emption_vulnerability: PreEmptionVulnerability::NotPreEmptable,
                        },
                        gbr_qos_flow_information: None,
                        reflective_qos_attribute: None,
                        additional_qos_information: None,
                        paging_policy_indicator: None,
                        reflective_qos_indicator: None,
                    },
                    qos_flow_mapping_indication: None,
                },
            ]),
            drb_data_forwarding_information_request: None,
            drb_inactivity_timer: None,
            pdcp_sn_status_information: None,
        }]),
    };
    let bearer_context_setup = BearerContextSetupRequest {
        gnb_cu_cp_ue_e1ap_id: GnbCuCpUeE1apId(ue.key),
        security_information: SecurityInformation {
            security_algorithm: SecurityAlgorithm {
                ciphering_algorithm: CipheringAlgorithm::Nea0,
                integrity_protection_algorithm: None,
            },
            up_securitykey: UpSecuritykey {
                encryption_key: EncryptionKey(vec![]),
                integrity_protection_key: None,
            },
        },
        ue_dl_aggregate_maximum_bit_rate,
        ue_dl_maximum_integrity_protected_data_rate: None,
        serving_plmn: PlmnIdentity(ue.plmn.clone()),
        activity_notification_level: ActivityNotificationLevel::PduSession,
        ue_inactivity_timer: None,
        bearer_context_status_change: None,
        system_bearer_context_setup_request:
            SystemBearerContextSetupRequest::NgRanBearerContextSetupRequest(
                NgRanBearerContextSetupRequest {
                    pdu_session_resource_to_setup_list: PduSessionResourceToSetupList(vec![item]),
                },
            ),
        ran_ue_id: None,
        gnb_du_id: None,
        trace_activation: None,
        npn_context_info: None,
        management_based_mdt_plmn_list: None,
        cho_initiation: None,
        additional_handover_info: None,
        direct_forwarding_path_availability: None,
        gnb_cu_up_ue_e1ap_id: None,
    };

    Ok(())
}
