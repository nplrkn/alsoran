//! pdu_session_resource_setup - AMF orders setup of PDU sessions and DRBs

use anyhow::Result;

use bitvec::prelude::*;
use e1ap::*;
use f1ap::{
    DrbsToBeSetupItem, DrbsToBeSetupList, GnbCuUeF1apId, QosInformation, UeContextSetupProcedure,
    UeContextSetupRequest,
};
use net::AperSerde;

use ngap::{
    PduSessionResourceFailedToSetupItemSuRes, PduSessionResourceFailedToSetupListSuRes,
    PduSessionResourceSetupItemSuReq, PduSessionResourceSetupItemSuRes,
    PduSessionResourceSetupListSuRes, PduSessionResourceSetupRequest,
    PduSessionResourceSetupRequestTransfer, PduSessionResourceSetupResponse,
};
use slog::debug;

use crate::{datastore::UeState, gnbcu_trait::Gnbcu};

use super::Workflow;

impl<'a, G: Gnbcu> Workflow<'a, G> {
    // Pdu session resource setup procedure.
    //
    // See documentation/session establishment.md
    //
    // 1.    Ngap PduSessionResourceSetupRequest(Nas) <<
    // 2. << E1ap BearerContextSetupRequest
    // 3. >> E1ap BearerContextSetupResponse
    // 4. << F1ap UeContextSetupRequest
    // 5. >> F1ap UeContextSetupResponse
    // 6. << E1ap BearerContextModificationRequest
    // 7. >> E1ap BearerContextModificationResponse
    // 8. << Dl Rrc Message Transfer + Rrc Reconfiguration + Nas PDU Session Establishment Accept
    // 9. >> Ul Rrc Message Transfer + Rrc Reconfiguration Complete
    // 8.    Pdu Session Resource Setup Response >>

    pub async fn pdu_session_resource_setup(
        &self,
        r: PduSessionResourceSetupRequest,
    ) -> PduSessionResourceSetupResponse {
        debug!(self.logger, "PduSessionResourceSetupRequest(Nas) << ");

        let (successful, unsuccessful) = self
            .pdu_session_resource_setup_inner(&r)
            .await
            .unwrap_or_else(|_| {
                (
                    Vec::new(),
                    r.pdu_session_resource_setup_list_su_req.0.iter().collect(),
                )
            });

        // TODO: this is doable without cloning the pdu_session_resource_setup_request_transfer.

        let pdu_session_resource_setup_list_su_res = if successful.is_empty() {
            None
        } else {
            Some(PduSessionResourceSetupListSuRes(
                successful
                    .into_iter()
                    .map(|x| PduSessionResourceSetupItemSuRes {
                        pdu_session_id: x.pdu_session_id,
                        pdu_session_resource_setup_response_transfer: x
                            .pdu_session_resource_setup_request_transfer
                            .clone(),
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
                            .pdu_session_resource_setup_request_transfer
                            .clone(),
                    })
                    .collect(),
            ))
        };

        debug!(self.logger, "PduSessionResourceSetupResponse >> ");
        PduSessionResourceSetupResponse {
            amf_ue_ngap_id: r.amf_ue_ngap_id,
            ran_ue_ngap_id: r.ran_ue_ngap_id,
            pdu_session_resource_setup_list_su_res,
            pdu_session_resource_failed_to_setup_list_su_res,
            criticality_diagnostics: None,
        }
    }

    pub async fn pdu_session_resource_setup_inner<'b>(
        &self,
        r: &'b PduSessionResourceSetupRequest,
    ) -> Result<(
        Vec<&'b PduSessionResourceSetupItemSuReq>,
        Vec<&'b PduSessionResourceSetupItemSuReq>,
    )> {
        // Retrieve UE context by ran_ue_ngap_id.
        debug!(self.logger, "Retrieve UE {:#010x}", r.ran_ue_ngap_id.0);
        let mut ue = self.retrieve(&r.ran_ue_ngap_id.0).await?;

        // Keep track of which sessions we managed to set up, via references into the original message.
        let mut unsuccessful = vec![];
        let mut successful = vec![];

        // Build PduSessionResourceToSetupItems.
        let mut items = vec![];
        for x in r.pdu_session_resource_setup_list_su_req.0.iter() {
            match self.build_setup_item(&ue, x) {
                Ok(item) => {
                    items.push(item);
                    successful.push(x);
                }
                Err(e) => {
                    debug!(self.logger, "Failed to build session setup item {:?}", e);
                    unsuccessful.push(x);
                }
            };
        }

        // TODO - the following functions hardcode a lot of things they shouldn't and will need work to signal session setup correctly.

        // Send BearerContextSetup to CU-UP.
        let bearer_context_setup = self.build_bearer_context_setup(&ue, items);
        debug!(self.logger, "<< BearerContextSetupRequest");
        let response = self
            .e1ap_request::<BearerContextSetupProcedure>(bearer_context_setup, self.logger)
            .await?;
        debug!(self.logger, ">> BearerContextSetupResponse");

        // Store CU-UP's UE ID.
        let gnb_cu_up_ue_e1ap_id = response.gnb_cu_up_ue_e1ap_id;
        ue.gnb_cu_up_ue_e1ap_id = Some(gnb_cu_up_ue_e1ap_id);

        // Send UeContextSetupRequest to DU.
        let ue_context_setup_request = self.build_ue_context_setup_request(&ue, None);
        self.log_message("<< UeContextSetupRequest");
        let ue_context_setup_response = self
            .f1ap_request::<UeContextSetupProcedure>(ue_context_setup_request, self.logger)
            .await?;
        self.log_message(">> UeContextSetupResponse");

        // Send BearerContextModification to CU-UP.
        let bearer_context_modification = self.build_bearer_context_modification(
            &ue,
            gnb_cu_up_ue_e1ap_id,
            ue_context_setup_response,
        );
        self.log_message("<< BearerContextMdificationRequest");
        let _response = self
            .e1ap_request::<BearerContextModificationProcedure>(
                bearer_context_modification,
                self.logger,
            )
            .await?;
        self.log_message(">> BearerContextModificationResponse");

        // Collect the Nas messages from the successful setups.
        // TODO - as per the similar comment in pdu_session_resource_setup(), we only need one copy of this data, so this code should be reorganized
        // so that it doesn't have to clone.
        let nas_messages = successful
            .iter()
            .filter_map(|x| x.pdu_session_nas_pdu.as_ref().map(|x| x.0.clone()))
            .collect();

        // Perform Rrc Reconfiguration including the Nas message from earlier.
        let rrc_transaction = self.new_rrc_transaction(&ue).await;
        let rrc_container = super::build_rrc::build_rrc_reconfiguration(3, Some(nas_messages))?;
        self.log_message("<< RrcReconfiguration");
        self.send_rrc_to_ue(&ue, f1ap::SrbId(1), rrc_container, self.logger)
            .await;
        let _rrc_reconfiguration_complete: rrc::UlDcchMessage = rrc_transaction.recv().await?;
        self.log_message(">> RrcReconfigurationComplete");

        // Write back UE.
        debug!(self.logger, "Store UE {:#010x}", ue.key);
        self.store(ue.key, ue, self.config().ue_ttl_secs).await?;

        Ok((successful, unsuccessful))
    }

    pub fn build_setup_item(
        &self,
        _ue: &UeState,
        r: &PduSessionResourceSetupItemSuReq,
    ) -> Result<PduSessionResourceToSetupItem> {
        let _session_params = PduSessionResourceSetupRequestTransfer::from_bytes(
            &r.pdu_session_resource_setup_request_transfer,
        )?;
        Ok(PduSessionResourceToSetupItem {
            pdu_session_id: PduSessionId(r.pdu_session_id.0),
            pdu_session_type: PduSessionType::Ipv4,
            snssai: Snssai {
                sst: r.s_nssai.sst.0.clone(),
                sd: r.s_nssai.sd.clone().map(|x| x.0),
            },
            security_indication: SecurityIndication {
                integrity_protection_indication: IntegrityProtectionIndication::Preferred,
                confidentiality_protection_indication:
                    ConfidentialityProtectionIndication::Preferred,
                maximum_i_pdatarate: None,
            },
            pdu_session_resource_dl_ambr: None,
            // TODO: Frunk transmogrify would be ideal
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
                            ngran_allocation_retention_priority:
                                NgranAllocationAndRetentionPriority {
                                    priority_level: PriorityLevel(1),
                                    pre_emption_capability:
                                        PreEmptionCapability::MayTriggerPreEmption,
                                    pre_emption_vulnerability:
                                        PreEmptionVulnerability::NotPreEmptable,
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
        })
    }

    pub fn build_bearer_context_setup(
        &self,
        ue: &UeState,
        items: Vec<PduSessionResourceToSetupItem>,
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
            serving_plmn: PlmnIdentity(self.config().plmn.clone()),
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
        &self,
        ue: &UeState,
        gnb_cu_up_ue_e1ap_id: GnbCuUpUeE1apId,
        _ue_context_setup_response: f1ap::UeContextSetupResponse,
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
            system_bearer_context_modification_request: None,
            ran_ue_id: None,
            gnb_du_id: None,
            activity_notification_level: None,
        }
    }

    fn build_ue_context_setup_request(
        &self,
        ue: &UeState,
        rrc_container: Option<f1ap::RrcContainer>,
    ) -> UeContextSetupRequest {
        // TODO: derive and use frunk transmogrify for the common ngap / f1ap structures seen here.

        UeContextSetupRequest {
            gnb_cu_ue_f1ap_id: GnbCuUeF1apId(ue.key),
            gnb_du_ue_f1ap_id: Some(ue.gnb_du_ue_f1ap_id),
            sp_cell_id: f1ap::NrCgi {
                plmn_identity: f1ap::PlmnIdentity(self.config().plmn.clone()),
                nr_cell_identity: f1ap::NrCellIdentity(bitvec![u8,Msb0;0;36]),
            },
            serv_cell_index: f1ap::ServCellIndex(0),
            sp_cell_ul_configured: None,
            cu_to_du_rrc_information: f1ap::CuToDuRrcInformation {
                cg_config_info: None,
                ue_capability_rat_container_list: None,
                meas_config: None,
            },
            candidate_sp_cell_list: None,
            drx_cycle: None,
            resource_coordination_transfer_container: None,
            s_cell_to_be_setup_list: None,
            srbs_to_be_setup_list: None,
            drbs_to_be_setup_list: Some(DrbsToBeSetupList(vec![DrbsToBeSetupItem {
                drb_id: f1ap::DrbId(1),
                qos_information: QosInformation::EutranQos(f1ap::EutranQos {
                    qci: f1ap::Qci(1),
                    allocation_and_retention_priority: f1ap::AllocationAndRetentionPriority {
                        priority_level: f1ap::PriorityLevel(1),
                        pre_emption_capability: f1ap::PreEmptionCapability::MayTriggerPreEmption,
                        pre_emption_vulnerability: f1ap::PreEmptionVulnerability::NotPreEmptable,
                    },
                    gbr_qos_information: None,
                }),
                uluptnl_information_to_be_setup_list: f1ap::UluptnlInformationToBeSetupList(vec![
                    f1ap::UluptnlInformationToBeSetupItem {
                        uluptnl_information: f1ap::UpTransportLayerInformation::GtpTunnel(
                            f1ap::GtpTunnel {
                                transport_layer_address: f1ap::TransportLayerAddress(
                                    bitvec![u8,Msb0;0,1,1,0],
                                ),
                                gtp_teid: f1ap::GtpTeid(vec![1, 2, 3, 4]),
                            },
                        ),
                    },
                ]),
                rlc_mode: f1ap::RlcMode::RlcUmBidirectional,
                ul_configuration: None,
                duplication_activation: None,
            }])),
            inactivity_monitoring_request: None,
            rat_frequency_priority_information: None,
            rrc_container,
            masked_imeisv: None, // r.masked_imeisv,
            serving_plmn: None,
            gnb_du_ue_ambr_ul: None,
            rrc_delivery_status_request: None,
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
        }
    }
}
