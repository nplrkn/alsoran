use crate::datastore::UeState;
use anyhow::Result;
use asn1_per::*;
use e1ap::*;
use ngap::{PduSessionResourceSetupItemSuReq, PduSessionResourceSetupRequestTransfer};
use xxap::{GtpTeid, GtpTunnel, PduSessionId};

// TODO: move to build_e1ap
pub fn build_e1_setup_item(
    _ue: &UeState,
    r: &PduSessionResourceSetupItemSuReq,
) -> Result<PduSessionResourceToSetupItem> {
    let snssai: xxap::Snssai = r.snssai.clone().into();
    let _session_params = PduSessionResourceSetupRequestTransfer::from_bytes(
        &r.pdu_session_resource_setup_request_transfer,
    )?;
    Ok(PduSessionResourceToSetupItem {
        pdu_session_id: PduSessionId(r.pdu_session_id.0),
        pdu_session_type: PduSessionType::Ipv4,
        snssai: snssai.into(),
        security_indication: SecurityIndication {
            integrity_protection_indication: IntegrityProtectionIndication::Preferred,
            confidentiality_protection_indication: ConfidentialityProtectionIndication::Preferred,
            maximum_i_pdatarate: None,
        },
        pdu_session_resource_dl_ambr: None,
        // TODO: get transport information from the request
        ng_ul_up_tnl_information: UpTnlInformation::GtpTunnel(GtpTunnel {
            transport_layer_address: "192.168.110.82".try_into()?,
            gtp_teid: GtpTeid([0, 0, 0, 1]),
        }),
        pdu_session_data_forwarding_information_request: None,
        pdu_session_inactivity_timer: None,
        existing_allocated_ng_dl_up_tnl_info: None,
        network_instance: None,
        drb_to_setup_list_ng_ran: DrbToSetupListNgRan(nonempty![DrbToSetupItemNgRan {
            drb_id: DrbId(1),
            sdap_configuration: SdapConfiguration {
                default_drb: DefaultDrb::True, // test
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
                pdcp_status_report_indication: None,
                additional_pdc_pduplication_information: None,
                ehc_parameters: None,
            },
            cell_group_information: CellGroupInformation(nonempty![CellGroupInformationItem {
                cell_group_id: CellGroupId(1),
                ul_configuration: None,
                dl_tx_stop: None,
                rat_type: None,
                number_of_tunnels: None,
            }]),
            qos_flow_information_to_be_setup: QosFlowQosParameterList(nonempty![
                QosFlowQosParameterItem {
                    qos_flow_identifier: QosFlowIdentifier(1),
                    qos_flow_level_qos_parameters: QosFlowLevelQosParameters {
                        qos_characteristics: QosCharacteristics::NonDynamic5qi(
                            NonDynamic5qiDescriptor {
                                five_qi: 1,
                                qos_priority_level: None,
                                averaging_window: None,
                                max_data_burst_volume: None,
                                cn_packet_delay_budget_downlink: None,
                                cn_packet_delay_budget_uplink: None,
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
                        qos_monitoring_request: None,
                        mcg_offered_gbr_qos_flow_info: None,
                        qos_monitoring_reporting_frequency: None,
                        qos_monitoring_disabled: None,
                    },
                    qos_flow_mapping_indication: None,
                    redundant_qos_flow_indicator: None,
                    tsc_traffic_characteristics: None,
                },
            ]),
            drb_data_forwarding_information_request: None,
            drb_inactivity_timer: None,
            pdcp_sn_status_information: None,
            drb_qos: None,
            daps_request_info: None,
            ignore_mapping_rule_indication: None,
        }]),
        common_network_instance: None,
        redundant_n_g_ul_up_tnl_information: None,
        redundant_common_network_instance: None,
        redundant_pdu_session_information: None,
    })
}

pub fn build_bearer_context_setup(
    ue: &UeState,
    serving_plmn: PlmnIdentity,
    items: NonEmpty<PduSessionResourceToSetupItem>,
) -> BearerContextSetupRequest {
    let ue_dl_aggregate_maximum_bit_rate = BitRate(1000);

    BearerContextSetupRequest {
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
        serving_plmn,
        activity_notification_level: ActivityNotificationLevel::PduSession,
        ue_inactivity_timer: None,
        bearer_context_status_change: None,
        system_bearer_context_setup_request:
            SystemBearerContextSetupRequest::NgRanBearerContextSetupRequest(
                NgRanBearerContextSetupRequest {
                    pdu_session_resource_to_setup_list: PduSessionResourceToSetupList(items),
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
    }
}

pub fn build_bearer_context_modification(
    ue: &UeState,
    gnb_cu_up_ue_e1ap_id: GnbCuUpUeE1apId,
    items: NonEmpty<PduSessionResourceToModifyItem>,
) -> BearerContextModificationRequest {
    // TODO incomplete - for example need to supply a system_bearer_context_modification_request
    // with DrbToModifyListNgRan containing the UpTransportLayerInformation received in the
    // DU's DrbsSetupList.

    BearerContextModificationRequest {
        gnb_cu_cp_ue_e1ap_id: GnbCuCpUeE1apId(ue.key),
        gnb_cu_up_ue_e1ap_id,
        security_information: None,
        ue_dl_aggregate_maximum_bit_rate: None,
        ue_dl_maximum_integrity_protected_data_rate: None,
        bearer_context_status_change: None,
        new_ul_tnl_information_required: None,
        ue_inactivity_timer: None,
        data_discard_required: None,
        system_bearer_context_modification_request: Some(
            SystemBearerContextModificationRequest::NgRanBearerContextModificationRequest(
                NgRanBearerContextModificationRequest {
                    pdu_session_resource_to_setup_mod_list: None,
                    pdu_session_resource_to_modify_list: Some(PduSessionResourceToModifyList(
                        items,
                    )),
                    pdu_session_resource_to_remove_list: None,
                },
            ),
        ),
        ran_ue_id: None,
        gnb_du_id: None,
        activity_notification_level: None,
    }
}

pub fn build_e1_modify_item(
    pdu_session_id: PduSessionId,
    gtp_tunnel: GtpTunnel,
) -> Result<PduSessionResourceToModifyItem> {
    Ok(PduSessionResourceToModifyItem {
        pdu_session_id,
        security_indication: None,
        pdu_session_resource_dl_ambr: None,
        ng_ul_up_tnl_information: Some(UpTnlInformation::GtpTunnel(gtp_tunnel.clone())),
        pdu_session_data_forwarding_information_request: None,
        pdu_session_data_forwarding_information: None,
        pdu_session_inactivity_timer: None,
        network_instance: None,
        drb_to_setup_list_ng_ran: None,
        drb_to_modify_list_ng_ran: None,
        drb_to_remove_list_ng_ran: None,
        snssai: None,
        common_network_instance: None,
        redundant_n_g_ul_up_tnl_information: None,
        redundant_common_network_instance: None,
        data_forwardingto_eutran_information_list: None,
    })
}
