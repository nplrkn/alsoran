//! pdu_session_resource_setup - AMF orders setup of PDU sessions and DRBs

use super::{GnbCuCp, Workflow};
use crate::datastore::UeState;
use anyhow::{anyhow, bail, ensure, Result};
use e1ap::*;
use f1ap::{CellGroupConfig, DrbsToBeSetupList, UeContextSetupProcedure};
use net::SerDes;
use ngap::{
    AssociatedQosFlowItem, AssociatedQosFlowList, PduSessionResourceFailedToSetupItemSuRes,
    PduSessionResourceFailedToSetupListSuRes, PduSessionResourceSetupItemSuReq,
    PduSessionResourceSetupItemSuRes, PduSessionResourceSetupListSuRes,
    PduSessionResourceSetupRequest, PduSessionResourceSetupRequestTransfer,
    PduSessionResourceSetupResponse, PduSessionResourceSetupResponseTransfer,
    QosFlowPerTnlInformation, UpTransportLayerInformation,
};
use slog::{debug, warn};
use xxap::*;

struct Stage1 {
    ngap_request: ngap::PduSessionResourceSetupItemSuReq,
}
struct Stage2 {
    stage1: Stage1,
    e1_setup_response: e1ap::PduSessionResourceSetupItem,
}
struct Stage3 {
    stage2: Stage2,
    f1_setup_response: f1ap::DrbsSetupItem,
}
struct Stage4 {
    stage3: Stage3,
    _e1_modify_response: e1ap::PduSessionResourceModifiedItem,
}

impl<'a, G: GnbCuCp> Workflow<'a, G> {
    async fn pdu_session_resource_setup_stage_1(
        &self,
        ue: &mut UeState,
        mut sessions: Vec<Stage1>,
    ) -> Vec<Stage2> {
        // TODO: convert each of these to subfunctions?

        // Build E1 PduSessionResourceToSetupItems.
        let mut items = vec![];
        for session in &sessions {
            match self.build_e1_setup_item(&ue, &session.ngap_request) {
                Ok(item) => items.push(item),
                // Case where E1 setup item failed
                Err(e) => {
                    warn!(self.logger, "Build E1 setup item failed {:?}", e);
                }
            }
        }

        if items.is_empty() {
            return vec![];
        }

        // Send BearerContextSetup to CU-UP.
        let bearer_context_setup = self.build_bearer_context_setup(&ue, items);
        debug!(self.logger, "<< BearerContextSetupRequest");
        let mut resource_setup_items = match self
            .e1ap_request::<BearerContextSetupProcedure>(bearer_context_setup, self.logger)
            .await
        {
            Ok(BearerContextSetupResponse {
                gnb_cu_up_ue_e1ap_id,
                system_bearer_context_setup_response:
                    SystemBearerContextSetupResponse::NgRanBearerContextSetupResponse(
                        NgRanBearerContextSetupResponse {
                            pdu_session_resource_setup_list: PduSessionResourceSetupList(x),
                            ..
                        },
                    ),
                ..
            }) => {
                // Success - store CU-UP's UE ID.
                ue.gnb_cu_up_ue_e1ap_id = Some(gnb_cu_up_ue_e1ap_id);

                x
            }
            Ok(m) => {
                warn!(
                    self.logger,
                    "BearerContextSetupRequest without NGRAN resource setup items: {:?}", m
                );
                return vec![];
            }
            Err(e) => {
                debug!(self.logger, "Failed bearer context setup {:?}", e);
                return vec![];
            }
        };
        debug!(self.logger, ">> BearerContextSetupResponse");

        // Rebuild the session results vec, adding in the new info from the UP.
        let mut new_sessions: Vec<Stage2> = vec![];
        for session in sessions.drain(..) {
            let pdu_session_id = session.ngap_request.pdu_session_id.0;
            let index = resource_setup_items
                .iter()
                .position(|item| item.pdu_session_id.0 == pdu_session_id);

            match index {
                // Success case
                Some(index) => new_sessions.push(Stage2 {
                    stage1: session,
                    e1_setup_response: resource_setup_items.swap_remove(index),
                }),
                // Case where the CU-UP failed (or didn't return) this session.
                None => {
                    warn!(
                        self.logger,
                        "Session {} failed bearer context setup", pdu_session_id
                    );
                }
            };
        }

        new_sessions
    }

    async fn pdu_session_resource_setup_stage_2(
        &self,
        ue: &UeState,
        mut sessions: Vec<Stage2>,
    ) -> Result<(Vec<Stage3>, CellGroupConfig)> {
        let mut items = vec![];
        for session in &sessions {
            let Stage2 {
                stage1:
                    Stage1 {
                        ngap_request:
                            PduSessionResourceSetupItemSuReq {
                                pdu_session_id,
                                snssai,
                                ..
                            },
                    },
                e1_setup_response:
                    PduSessionResourceSetupItem {
                        ng_dl_up_tnl_information: UpTnlInformation::GtpTunnel(gtp_tunnel),
                        ..
                    },
            } = session;
            let pdu_session_id = pdu_session_id.0;

            // Pass the transport address of the CU-UP to the DU.
            let snssai: xxap::Snssai = snssai.clone().into();
            match super::build_f1ap::build_drb_to_be_setup_item(
                f1ap::DrbId(pdu_session_id),
                snssai.into(),
                gtp_tunnel.clone(),
            ) {
                Ok(drb_setup_item) => items.push(drb_setup_item),
                Err(e) => warn!(self.logger, "Failed to build Drb item - {:?}", e),
            }
        }

        ensure!(!items.is_empty(), "No Drb items built successfully");
        let ue_context_setup_request = super::build_f1ap::build_ue_context_setup_request(
            self.gnb_cu_cp,
            &ue,
            Some(DrbsToBeSetupList(items)),
            None,
        )?;

        // Send UeContextSetupRequest to DU.
        self.log_message("<< UeContextSetupRequest");
        let ue_context_setup_response = self
            .f1ap_request::<UeContextSetupProcedure>(ue_context_setup_request, self.logger)
            .await?;
        self.log_message(">> UeContextSetupResponse");

        // TS38.473, 8.3.1.2: "If the CellGroupConfig IE is included in the DU to CU RRC Information IE contained in the UE CONTEXT SETUP RESPONSE message,
        // the gNB-CU shall perform RRC Reconfiguration or RRC connection resume as described in TS 38.331 [8]. The CellGroupConfig IE shall
        // transparently be signaled to the UE as specified in TS 38.331 [8]."
        let cell_group_config = ue_context_setup_response
            .du_to_cu_rrc_information
            .cell_group_config
            .0;
        let cell_group_config = CellGroupConfig(cell_group_config);

        // TODO - can this be made generic with previous function

        // Extract the session items from the response.
        let mut drbs_setup_list = match ue_context_setup_response.drbs_setup_list {
            Some(x) => x.0,
            _ => {
                bail!("UeContextSetupResponse without DRB setup list");
            }
        };

        // Rebuild the session results vec, adding in the new info from the UP.
        let mut new_sessions: Vec<Stage3> = vec![];
        for session in sessions.drain(..) {
            let pdu_session_id = session.stage1.ngap_request.pdu_session_id.0;
            let index = drbs_setup_list
                .iter()
                .position(|item| item.drb_id.0 == pdu_session_id);

            match index {
                // Success case
                Some(index) => new_sessions.push(Stage3 {
                    stage2: session,
                    f1_setup_response: drbs_setup_list.swap_remove(index),
                }),
                // Case where the CU-UP failed (or didn't return) this session.
                None => {
                    warn!(
                        self.logger,
                        "Session {} failed bearer context setup", pdu_session_id
                    );
                }
            };
        }

        Ok((new_sessions, cell_group_config))
    }

    async fn pdu_session_resource_setup_stage_3(
        &self,
        ue: &UeState,
        mut sessions: Vec<Stage3>,
    ) -> Vec<Stage4> {
        // TODO: convert each of these to subfunctions?

        // Build E1 PduSessionResourceToSetupItems.
        let mut items = vec![];
        for session in &sessions {
            match self.build_e1_modify_item(&ue, session) {
                Ok(item) => items.push(item),
                // Case where E1 setup item failed
                Err(e) => {
                    warn!(self.logger, "Build E1 setup item failed {:?}", e);
                }
            }
        }

        if items.is_empty() {
            warn!(self.logger, "No sessions left");
            return vec![];
        }

        let Some(gnb_cu_up_ue_e1ap_id) = ue.gnb_cu_up_ue_e1ap_id else {
            warn!(self.logger, "No E1AP ID on UE");
            return vec![]
        };

        // Send BearerContextModify to CU-UP.
        let bearer_context_modification =
            self.build_bearer_context_modification(&ue, gnb_cu_up_ue_e1ap_id, items);
        debug!(self.logger, "<< BearerContextSetupRequest");
        let mut resource_modify_items = match self
            .e1ap_request::<BearerContextModificationProcedure>(
                bearer_context_modification,
                self.logger,
            )
            .await
        {
            Ok(BearerContextModificationResponse {
                system_bearer_context_modification_response:
                    Some(SystemBearerContextModificationResponse::NgRanBearerContextModificationResponse(
                        NgRanBearerContextModificationResponse {
                            pdu_session_resource_modified_list:
                                Some(PduSessionResourceModifiedList(x)),
                            ..
                        },
                    )),
                ..
            }) => x,
            Ok(m) => {
                warn!(
                    self.logger,
                    "BearerContextModificationResponse without resource modify items: {:?}", m
                );
                return vec![];
            },
            Err(e) => {
                debug!(self.logger, "Failed bearer context modify {:?}", e);
                return vec![];
            }
        };
        debug!(self.logger, ">> BearerContextSetupResponse");

        // Rebuild the session results vec, adding in the new info from the UP.
        let mut new_sessions: Vec<Stage4> = vec![];
        for session in sessions.drain(..) {
            let pdu_session_id = session.stage2.stage1.ngap_request.pdu_session_id.0;
            let index = resource_modify_items
                .iter()
                .position(|item| item.pdu_session_id.0 == pdu_session_id);

            match index {
                // Success case
                Some(index) => new_sessions.push(Stage4 {
                    stage3: session,
                    _e1_modify_response: resource_modify_items.swap_remove(index),
                }),
                // Case where the CU-UP failed (or didn't return) this session.
                None => {
                    warn!(
                        self.logger,
                        "Session {} failed bearer context setup", pdu_session_id
                    );
                }
            };
        }

        new_sessions
    }

    async fn pdu_session_resource_setup_stage_4(
        &self,
        ue: &UeState,
        sessions: Vec<Stage4>,
        cell_group_config: f1ap::CellGroupConfig,
    ) -> Result<Vec<Stage4>> {
        // Collect the Nas messages from the successful setups.
        // TODO - as per the similar comment in pdu_session_resource_setup(), we only need one copy of this data, so this code should be reorganized
        // so that it doesn't have to clone.
        let nas_messages: Vec<Vec<u8>> = sessions
            .iter()
            .filter_map(|x| {
                x.stage3
                    .stage2
                    .stage1
                    .ngap_request
                    .pdu_session_nas_pdu
                    .as_ref()
                    .map(|x| x.0.clone())
            })
            .collect();

        debug!(self.logger, "Nas messages is {:?}", nas_messages);

        // Perform Rrc Reconfiguration including the Nas messages from earlier and the cell group config received from the DU.
        let rrc_transaction = self.new_rrc_transaction(&ue).await;
        let nas_messages = if nas_messages.is_empty() {
            None
        } else {
            Some(nas_messages)
        };
        let rrc_container =
            super::build_rrc::build_rrc_reconfiguration(3, nas_messages, cell_group_config.0)?;
        self.log_message("<< RrcReconfiguration");
        self.send_rrc_to_ue(&ue, f1ap::SrbId(1), rrc_container, self.logger)
            .await;
        let _rrc_reconfiguration_complete: rrc::UlDcchMessage = rrc_transaction.recv().await?;
        self.log_message(">> RrcReconfigurationComplete");

        Ok(sessions)
    }

    async fn pdu_session_resource_setup_stage_5(
        &self,
        _ue: &UeState,
        mut sessions: Vec<Stage4>,
    ) -> Result<Vec<PduSessionResourceSetupItemSuRes>> {
        let mut new_sessions = vec![];
        for session in sessions.drain(..) {
            let UpTnlInformation::GtpTunnel(gtp_tunnel) = session
                .stage3
                .stage2
                .e1_setup_response
                .ng_dl_up_tnl_information;
            let new_session = PduSessionResourceSetupItemSuRes {
                pdu_session_id: session.stage3.stage2.stage1.ngap_request.pdu_session_id,
                pdu_session_resource_setup_response_transfer:
                    PduSessionResourceSetupResponseTransfer {
                        dl_qos_flow_per_tnl_information: QosFlowPerTnlInformation {
                            up_transport_layer_information: UpTransportLayerInformation::GtpTunnel(
                                gtp_tunnel,
                            ),
                            associated_qos_flow_list: AssociatedQosFlowList(vec![
                                AssociatedQosFlowItem {
                                    qos_flow_identifier: ngap::QosFlowIdentifier(1),
                                    qos_flow_mapping_indication: None,
                                    current_qos_para_set_index: None,
                                },
                            ]),
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
            };
            new_sessions.push(new_session)
        }
        Ok(new_sessions)
    }

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

        let amf_ue_ngap_id = r.amf_ue_ngap_id;
        let ran_ue_ngap_id = r.ran_ue_ngap_id;

        // Save off the sessions IDs in case of error.  In theory we ought to be able to
        // accumulate different errors to different sessions over the course of the following
        // processing but the code is not currently sophisticated enough to do that.  Instead
        // it just tracks the successful ones.
        let session_ids: Vec<u8> = r
            .pdu_session_resource_setup_list_su_req
            .0
            .iter()
            .map(|item| item.pdu_session_id.0)
            .collect();

        // Go through all the stages of session resource setup.  If all goes well, the
        // successfully set up sessions will pop out the other side.
        let sessions = match self.pdu_session_resource_setup_stages(r).await {
            Ok(sessions) => sessions,
            Err(e) => {
                warn!(self.logger, "Error processing session setup - {}", e);
                vec![]
            }
        };

        let failed_session_ids: Vec<u8> = session_ids
            .into_iter()
            .filter(|x| sessions.iter().any(|item| item.pdu_session_id.0 == *x))
            .collect();

        let pdu_session_resource_setup_list_su_res = if sessions.is_empty() {
            None
        } else {
            Some(PduSessionResourceSetupListSuRes(sessions))
        };

        let pdu_session_resource_failed_to_setup_list_su_res = if failed_session_ids.is_empty() {
            None
        } else {
            Some(PduSessionResourceFailedToSetupListSuRes(
                failed_session_ids
                    .iter()
                    .map(|x| PduSessionResourceFailedToSetupItemSuRes {
                        pdu_session_id: ngap::PduSessionId(*x),
                        pdu_session_resource_setup_unsuccessful_transfer: vec![],
                    })
                    .collect(),
            ))
        };

        debug!(self.logger, "PduSessionResourceSetupResponse >> ");
        PduSessionResourceSetupResponse {
            amf_ue_ngap_id,
            ran_ue_ngap_id,
            pdu_session_resource_setup_list_su_res,
            pdu_session_resource_failed_to_setup_list_su_res,
            criticality_diagnostics: None,
        }
    }

    pub async fn pdu_session_resource_setup_stages(
        &self,
        mut r: PduSessionResourceSetupRequest,
    ) -> Result<Vec<PduSessionResourceSetupItemSuRes>> {
        debug!(self.logger, "Retrieve UE {:#010x}", r.ran_ue_ngap_id.0);
        let mut ue = self.retrieve(&r.ran_ue_ngap_id.0).await?;

        let sessions = r
            .pdu_session_resource_setup_list_su_req
            .0
            .drain(..)
            .map(|item| Stage1 { ngap_request: item })
            .collect();

        // E1 BearerContextSetupRequest.
        let sessions = self
            .pdu_session_resource_setup_stage_1(&mut ue, sessions)
            .await;

        // F1 UeContextSetupRequest
        let (sessions, cell_group_config) = self
            .pdu_session_resource_setup_stage_2(&mut ue, sessions)
            .await?;

        // E1 BearerContextModifyRequest.
        let sessions = self
            .pdu_session_resource_setup_stage_3(&mut ue, sessions)
            .await;

        // RRC Reconfiguration.
        let sessions = self
            .pdu_session_resource_setup_stage_4(&mut ue, sessions, cell_group_config)
            .await?;

        // Production of NGAP setup responses.
        let sessions = self
            .pdu_session_resource_setup_stage_5(&mut ue, sessions)
            .await?;

        // Write back UE.
        debug!(self.logger, "Store UE {:#010x}", ue.key);
        self.store(ue.key, ue, self.config().ue_ttl_secs).await?;

        Ok(sessions)
    }

    // TODO: move to build_e1ap
    pub fn build_e1_setup_item(
        &self,
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
                confidentiality_protection_indication:
                    ConfidentialityProtectionIndication::Preferred,
                maximum_i_pdatarate: None,
            },
            pdu_session_resource_dl_ambr: None,
            // TODO: get transport information from the request
            ng_ul_up_tnl_information: UpTnlInformation::GtpTunnel(GtpTunnel {
                transport_layer_address: "192.168.110.82".try_into()?,
                gtp_teid: GtpTeid(vec![0, 0, 0, 1]),
            }),
            pdu_session_data_forwarding_information_request: None,
            pdu_session_inactivity_timer: None,
            existing_allocated_ng_dl_up_tnl_info: None,
            network_instance: None,
            drb_to_setup_list_ng_ran: DrbToSetupListNgRan(vec![DrbToSetupItemNgRan {
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
                cell_group_information: CellGroupInformation(vec![CellGroupInformationItem {
                    cell_group_id: CellGroupId(1),
                    ul_configuration: None,
                    dl_tx_stop: None,
                    rat_type: None,
                    number_of_tunnels: None,
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
                                    cn_packet_delay_budget_downlink: None,
                                    cn_packet_delay_budget_uplink: None,
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
        items: Vec<PduSessionResourceToModifyItem>,
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

    fn build_e1_modify_item(
        &self,
        _ue: &UeState,
        session: &Stage3,
    ) -> Result<PduSessionResourceToModifyItem> {
        let pdu_session_id =
            e1ap::PduSessionId(session.stage2.stage1.ngap_request.pdu_session_id.0);
        let tnl_setup_list = &session
            .f1_setup_response
            .dluptnl_information_to_be_setup_list
            .0;

        ensure!(!tnl_setup_list.is_empty());

        let f1ap::UpTransportLayerInformation::GtpTunnel(gtp_tunnel) = &tnl_setup_list
            .first()
            .ok_or_else(|| anyhow!("No GTP tunnel information from DU"))?
            .dluptnl_information;

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
}
