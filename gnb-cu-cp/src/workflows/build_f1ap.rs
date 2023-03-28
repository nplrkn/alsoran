//! build_f1ap - construction of F1AP messages

use super::GnbCuCp;
use crate::datastore::UeState;
use anyhow::Result;
use asn1_per::SerDes;
use f1ap::*;
use rrc::{
    AccessStratumRelease, BandNr, FeatureSetDownlinkPerCc, FeatureSetUplinkPerCc, FeatureSets,
    FreqBandIndicatorNr, ModulationOrder, PdcpParameters, PhyParameters, RfParameters,
    SupportedBandwidth, SupportedRohcProfiles, UeCapabilityRatContainer, UeNrCapability,
};

fn build_drb_to_be_setup_item(
    drb_id: DrbId,
    snssai: Snssai,
    gtp_tunnel: GtpTunnel,
) -> Result<DrbsToBeSetupItem> {
    // TODO - removing hardcoded values

    Ok(DrbsToBeSetupItem {
        drb_id,
        qos_information: QosInformation::QosInformationExtIEs(QosInformationExtIEs {
            drb_information: DrbInformation {
                drb_qos: QosFlowLevelQosParameters {
                    qos_characteristics: QosCharacteristics::NonDynamic5qi(
                        NonDynamic5qiDescriptor {
                            five_qi: 9,
                            qos_priority_level: None,
                            averaging_window: None,
                            max_data_burst_volume: None,
                            cn_packet_delay_budget_downlink: None,
                            cn_packet_delay_budget_uplink: None,
                        },
                    ),
                    ngran_allocation_retention_priority: NgranAllocationAndRetentionPriority {
                        priority_level: PriorityLevel(14),
                        pre_emption_capability: PreEmptionCapability::MayTriggerPreEmption,
                        pre_emption_vulnerability: PreEmptionVulnerability::NotPreEmptable,
                    },
                    gbr_qos_flow_information: None,
                    reflective_qos_attribute: None,
                    pdu_session_id: Some(PduSessionId(1)),
                    ulpdu_session_aggregate_maximum_bit_rate: None,
                    qos_monitoring_request: None,
                },
                snssai,
                notification_control: None,
                flows_mapped_to_drb_list: FlowsMappedToDrbList(vec![FlowsMappedToDrbItem {
                    qos_flow_identifier: QosFlowIdentifier(0),
                    qos_flow_level_qos_parameters: QosFlowLevelQosParameters {
                        qos_characteristics: QosCharacteristics::NonDynamic5qi(
                            NonDynamic5qiDescriptor {
                                five_qi: 9,
                                qos_priority_level: None,
                                averaging_window: None,
                                max_data_burst_volume: None,
                                cn_packet_delay_budget_downlink: None,
                                cn_packet_delay_budget_uplink: None,
                            },
                        ),
                        ngran_allocation_retention_priority: NgranAllocationAndRetentionPriority {
                            priority_level: PriorityLevel(14),
                            pre_emption_capability: PreEmptionCapability::MayTriggerPreEmption,
                            pre_emption_vulnerability: PreEmptionVulnerability::NotPreEmptable,
                        },
                        gbr_qos_flow_information: None,
                        reflective_qos_attribute: None,
                        pdu_session_id: None,
                        ulpdu_session_aggregate_maximum_bit_rate: None,
                        qos_monitoring_request: None,
                    },
                    qos_flow_mapping_indication: None,
                    tsc_traffic_characteristics: None,
                }]),
            },
        }),
        uluptnl_information_to_be_setup_list: UluptnlInformationToBeSetupList(vec![
            UluptnlInformationToBeSetupItem {
                uluptnl_information: UpTransportLayerInformation::GtpTunnel(gtp_tunnel),
                bh_info: None,
            },
        ]),
        rlc_mode: RlcMode::RlcUmBidirectional,
        ul_configuration: None,
        duplication_activation: None,
        dc_based_duplication_configured: None,
        dc_based_duplication_activation: None,
        dlpdcpsn_length: None,
        ulpdcpsn_length: None,
        additional_pdcp_duplication_tnl_list: None,
        rlc_duplication_information: None,
    })
}

fn build_scell_to_be_setup_item(nr_cgi: NrCgi) -> SCellToBeSetupItem {
    SCellToBeSetupItem {
        s_cell_id: nr_cgi,
        s_cell_index: SCellIndex(1), // TODO
        s_cell_ul_configured: None,
        serving_cell_mo: None,
    }
}

pub fn build_ue_context_setup_request<G: GnbCuCp>(
    _gnb_cu_cp: &G,
    _r: &ngap::InitialContextSetupRequest,
    ue: &UeState,
    rrc_container: Option<f1ap::RrcContainer>,
) -> Result<UeContextSetupRequest> {
    // TODO: derive and use frunk for the common ngap / f1ap structures seen here?

    // Build a Ue Context Setup Request similar to the one sent by the O-RAN-SC O-DU's CU Stub to the ODU.
    // This has one SRB and two DRBs.
    let first_gtp_tunnel = GtpTunnel {
        transport_layer_address: TransportLayerAddress(net::ip_bits_from_string("192.168.130.82")?),
        gtp_teid: GtpTeid(vec![0, 0, 0, 1]),
    };
    let first_slice = Snssai {
        sst: vec![1],
        sd: Some(vec![2, 3, 4]),
    };
    let first_drb_to_setup_item =
        build_drb_to_be_setup_item(DrbId(1), first_slice, first_gtp_tunnel)?;

    let second_gtp_tunnel = GtpTunnel {
        transport_layer_address: TransportLayerAddress(net::ip_bits_from_string("192.168.130.82")?),
        gtp_teid: GtpTeid(vec![0, 0, 0, 2]),
    };
    let second_slice = Snssai {
        sst: vec![5],
        sd: Some(vec![6, 7, 8]),
    };
    let second_drb_to_setup_item =
        build_drb_to_be_setup_item(DrbId(2), second_slice, second_gtp_tunnel)?;

    Ok(UeContextSetupRequest {
        gnb_cu_ue_f1ap_id: GnbCuUeF1apId(ue.key),
        gnb_du_ue_f1ap_id: Some(ue.gnb_du_ue_f1ap_id),
        sp_cell_id: ue.nr_cgi.clone(),
        serv_cell_index: f1ap::ServCellIndex(0),
        sp_cell_ul_configured: Some(CellUlConfigured::None),
        cu_to_du_rrc_information: f1ap::CuToDuRrcInformation {
            cg_config_info: None,
            ue_capability_rat_container_list: Some(UeCapabilityRatContainerList(
                UeCapabilityRatContainer {
                    rat_type: rrc::RatType::Nr,
                    ue_capability_rat_container: UeNrCapability {
                        access_stratum_release: AccessStratumRelease::Rel15,
                        pdcp_parameters: PdcpParameters {
                            supported_rohc_profiles: SupportedRohcProfiles {
                                profile0x_0000: false,
                                profile0x_0001: false,
                                profile0x_0002: false,
                                profile0x_0003: false,
                                profile0x_0004: false,
                                profile0x_0006: false,
                                profile0x_0101: false,
                                profile0x_0102: false,
                                profile0x_0103: false,
                                profile0x_0104: false,
                            },
                            max_number_rohc_context_sessions:
                                rrc::MaxNumberRohcContextSessions::Cs2,
                            uplink_only_rohc_profiles: None,
                            continue_rohc_context: None,
                            out_of_order_delivery: None,
                            short_sn: None,
                            pdcp_duplication_srb: None,
                            pdcp_duplication_mcg_or_scg_drb: None,
                        },
                        rlc_parameters: None,
                        mac_parameters: None,
                        phy_parameters: PhyParameters {
                            phy_parameters_common: None,
                            phy_parameters_xdd_diff: None,
                            phy_parameters_frx_diff: None,
                            phy_parameters_fr1: None,
                            phy_parameters_fr2: None,
                        },
                        rf_parameters: RfParameters {
                            supported_band_list_nr: vec![BandNr {
                                band_nr: FreqBandIndicatorNr(1),
                                modified_mpr_behaviour: None,
                                mimo_parameters_per_band: None,
                                extended_cp: None,
                                multiple_tci: None,
                                bwp_without_restriction: None,
                                bwp_same_numerology: None,
                                bwp_diff_numerology: None,
                                cross_carrier_scheduling_same_scs: None,
                                pdsch_256qam_fr2: None,
                                pusch_256qam: None,
                                ue_power_class: None,
                                rate_matching_lte_crs: None,
                                channel_b_ws_dl: None,
                                channel_b_ws_ul: None,
                            }],
                            supported_band_combination_list: None,
                            applied_freq_band_list_filter: None,
                        },
                        meas_and_mob_parameters: None,
                        fdd_add_ue_nr_capabilities: None,
                        tdd_add_ue_nr_capabilities: None,
                        fr_1_add_ue_nr_capabilities: None,
                        fr_2_add_ue_nr_capabilities: None,
                        feature_sets: Some(FeatureSets {
                            feature_sets_downlink: None,
                            feature_sets_downlink_per_cc: Some(vec![FeatureSetDownlinkPerCc {
                                supported_subcarrier_spacing_dl: rrc::SubcarrierSpacing::KHz15,
                                supported_bandwidth_dl: SupportedBandwidth::Fr1(rrc::Fr1_2::Mhz20),
                                channel_bw_90mhz: None,
                                max_number_mimo_layers_pdsch: None,
                                supported_modulation_order_dl: Some(ModulationOrder::Qam64),
                            }]),
                            feature_sets_uplink: None,
                            feature_sets_uplink_per_cc: Some(vec![FeatureSetUplinkPerCc {
                                supported_subcarrier_spacing_ul: rrc::SubcarrierSpacing::KHz15,
                                supported_bandwidth_ul: SupportedBandwidth::Fr1(rrc::Fr1_2::Mhz20),
                                channel_bw_90mhz: None,
                                supported_modulation_order_ul: Some(ModulationOrder::Qam16),
                                mimo_cb_pusch: None,
                                max_number_mimo_layers_non_cb_pusch: None,
                            }]),
                        }),
                        feature_set_combinations: None,
                        late_non_critical_extension: None,
                        non_critical_extension: None,
                    }
                    .into_bytes()?,
                }
                .into_bytes()?,
            )),
            meas_config: None,
            handover_preparation_information: None,
            cell_group_config: None,
            measurement_timing_configuration: None,
            ue_assistance_information: None,
            cg_config: None,
            ue_assistance_information_eutra: None,
        },
        candidate_sp_cell_list: None,
        drx_cycle: None,
        resource_coordination_transfer_container: None,
        s_cell_to_be_setup_list: Some(SCellToBeSetupList(vec![build_scell_to_be_setup_item(
            ue.nr_cgi.clone(),
        )])),
        srbs_to_be_setup_list: Some(SrbsToBeSetupList(vec![SrbsToBeSetupItem {
            srb_id: SrbId(2),
            duplication_indication: None,
            additional_duplication_indication: None,
        }])),
        drbs_to_be_setup_list: Some(DrbsToBeSetupList(vec![
            first_drb_to_setup_item,
            second_drb_to_setup_item,
        ])),
        inactivity_monitoring_request: None,
        rat_frequency_priority_information: None,
        rrc_container,
        masked_imeisv: None, // r.masked_imeisv,
        serving_plmn: None,
        gnb_du_ue_ambr_ul: Some(BitRate(993522893)),
        rrc_delivery_status_request: Some(RrcDeliveryStatusRequest::True),
        resource_coordination_transfer_information: None,
        serving_cell_mo: None,
        new_gnb_cu_ue_f1ap_id: None,
        ran_ue_id: None,
        trace_activation: None,
        additional_rrm_priority_index: None,
        bh_channels_to_be_setup_list: None,
        configured_bap_address: None,
        nr_v2x_services_authorized: None, // r.nr_v2x_services_authorized,
        ltev2x_services_authorized: None, // r.ltev2x_services_authorized,
        nr_ue_sidelink_aggregate_maximum_bitrate: None, // r.nr_ue_sidelink_aggregate_maximum_bitrate,
        lte_ue_sidelink_aggregate_maximum_bitrate: None, // r.lte_ue_sidelink_aggregate_maximum_bitrate,
        pc5_link_ambr: None, // r.pc5_qos_parameters.and_then(|x| x.pc_5_link_aggregate_bit_rates),
        sl_drbs_to_be_setup_list: None,
        conditional_inter_du_mobility_information: None,
        management_based_mdt_plmn_list: None,
        serving_nid: None,
        f1c_transfer_path: None,
    })
}
