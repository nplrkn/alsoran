//! pdu_session_resource_setup - AMF orders setup of PDU sessions and DRBs

use super::{GnbCuCp, Workflow};
use crate::datastore::UeState;
use anyhow::{anyhow, Result};
use e1ap::*;
use f1ap::{DrbsToBeSetupList, UeContextSetupProcedure, UeContextSetupResponse};
use net::SerDes;
use ngap::{
    PduSessionResourceFailedToSetupItemSuRes, PduSessionResourceFailedToSetupListSuRes,
    PduSessionResourceSetupItemSuReq, PduSessionResourceSetupItemSuRes,
    PduSessionResourceSetupListSuRes, PduSessionResourceSetupRequest,
    PduSessionResourceSetupRequestTransfer, PduSessionResourceSetupResponse,
    PduSessionResourceSetupResponseTransfer,
};
use slog::{debug, warn};
use xxap::*;

// We start with a list of session to set up.
// At each stage we might bail out with a cause affecting all sessions.
// Or we might knock out 1 session only.
// Each stage has a new kind of result.
// We always need to refer back to the original ask.

// So

struct SessionResult<S> {
    pdu_session_id: u8,
    result: Result<S, ngap::Cause>,
}
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
    e1_modify_response: e1ap::PduSessionResourceModifiedList,
}

impl<'a, G: GnbCuCp> Workflow<'a, G> {
    pub async fn pdu_session_resource_setup_stage_1(
        &self,
        ue: &mut UeState,
        mut sessions: Vec<SessionResult<Stage1>>,
    ) -> Vec<SessionResult<Stage2>> {
        // TODO: convert each of these to subfunctions?

        // Build E1 PduSessionResourceToSetupItems.
        let mut items = vec![];
        for session in &mut sessions {
            match session
                .result
                .as_ref()
                .map(|s| self.build_e1_setup_item(&ue, &s.ngap_request))
            {
                Ok(Ok(item)) => items.push(item),
                // Case where E1 setup item failed
                Ok(Err(_)) => {
                    session.result = Err(ngap::Cause::Protocol(
                        ngap::CauseProtocol::AbstractSyntaxErrorFalselyConstructedMessage,
                    ))
                }
                // Case where the session had already failed
                Err(e) => (),
            }
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
                vec![]
            }
            Err(e) => {
                debug!(self.logger, "Failed bearer context setup {:?}", e);
                vec![]
            }
        };
        debug!(self.logger, ">> BearerContextSetupResponse");

        // Rebuild the session results vec, adding in the new info from the UP.
        let mut new_sessions: Vec<SessionResult<Stage2>> = vec![];
        for session in sessions.drain(..) {
            let pdu_session_id = session.pdu_session_id;
            let index = resource_setup_items
                .iter()
                .position(|item| item.pdu_session_id.0 == pdu_session_id);

            let result = match (session.result, index) {
                // Success case
                (Ok(stage1), Some(index)) => Ok(Stage2 {
                    stage1,
                    e1_setup_response: resource_setup_items.swap_remove(index),
                }),
                // Case where the session had already failed even before we talked to the CU-UP
                (Err(cause), _) => Err(cause),
                // Case where the CU-UP failed (or didn't return) this session.
                (Ok(_), None) => {
                    warn!(
                        self.logger,
                        "Session {} failed bearer context setup", pdu_session_id
                    );
                    Err(ngap::Cause::RadioNetwork(
                        ngap::CauseRadioNetwork::Unspecified,
                    ))
                }
            };
            new_sessions.push(SessionResult {
                pdu_session_id,
                result,
            });
        }

        new_sessions
    }

    pub async fn pdu_session_resource_setup_stage_2(
        &self,
        ue: &UeState,
        sessions: Vec<SessionResult<Stage2>>,
    ) -> Vec<SessionResult<Stage3>> {
        let mut items = vec![];
        for session in sessions {
            match session {
                SessionResult {
                    pdu_session_id,
                    result:
                        Ok(Stage2 {
                            stage1:
                                Stage1 {
                                    ngap_request: PduSessionResourceSetupItemSuReq { snssai, .. },
                                },
                            e1_setup_response:
                                PduSessionResourceSetupItem {
                                    ng_dl_up_tnl_information:
                                        UpTnlInformation::GtpTunnel(gtp_tunnel),
                                    ..
                                },
                        }),
                } => {
                    // Pass the transport address of the CU-UP to the DU.
                    match super::build_f1ap::build_drb_to_be_setup_item(
                        f1ap::DrbId(pdu_session_id),
                        snssai,
                        gtp_tunnel,
                    ) {
                        Ok(drb_setup_item) => items.push(drb_setup_item),
                        Err(e) => warn!(self.logger, "Failed to build Drb item - {:?}", e),
                    }
                }
                _ => {
                    warn!(
                        self.logger,
                        "Problem processing session setup result from CU-UP"
                    );

                    // We don't ask the DU to set up a DRB in this case.  We do not currently have a way of
                    // storing and returning a specific error cause at this point.
                }
            }
        }

        // Send UeContextSetupRequest to DU.
        let ue_context_setup_response = if !items.is_empty() {
            match super::build_f1ap::build_ue_context_setup_request(
                self.gnb_cu_cp,
                &ue,
                Some(DrbsToBeSetupList(items)),
                None,
            ) {
                Err(e) => Err(anyhow!("UeContextSetupRequest build failed - {:?}", e)),
                Ok(ue_context_setup_request) => {
                    self.log_message("<< UeContextSetupRequest");
                    self.f1ap_request::<UeContextSetupProcedure>(
                        ue_context_setup_request,
                        self.logger,
                    )
                    .await
                    .map_err(|e| anyhow!("UeContextSetupRequest request failed - {:?}", e))
                }
            }
        } else {
            Err(anyhow!("No remaining DRBs to be set up"))
        };

        match ue_context_setup_response {
            Ok(UeContextSetupResponse { .. }) => {
                self.log_message(">> UeContextSetupResponse");

                todo!()
            }
            Err(e) => {
                // Replace
                todo!()
            }
        }

        todo!()
    }

    pub async fn pdu_session_resource_setup_stage_3(
        &self,
        ue: &UeState,
        sessions: Vec<Result<Stage3, ngap::Cause>>,
    ) -> Vec<Result<Stage4, ngap::Cause>> {
        todo!()
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

        // The first stage is to carry out E1 bearer context setup.  This produces an E1AP PduSessionResourceSetupItem
        // for each successful result.  Start by assuming failure to look up the UE.
        let num_sessions = r.pdu_session_resource_setup_list_su_req.0.len();
        let mut session_results: Vec<Result<PduSessionResourceSetupItem, ngap::Cause>> = vec![
                Err(ngap::Cause::RadioNetwork(
                    ngap::CauseRadioNetwork::UnknownLocalUeNgapId
                ));
                num_sessions
            ];

        'setup_stages: {
            debug!(self.logger, "Retrieve UE {:#010x}", r.ran_ue_ngap_id.0);
            let mut ue = match self.retrieve(&r.ran_ue_ngap_id.0).await {
                Ok(ue) => ue,
                Err(_) => break 'setup_stages,
            };

            // Build E1 PduSessionResourceToSetupItems.
            let mut items = vec![];
            for (idx, x) in r
                .pdu_session_resource_setup_list_su_req
                .0
                .iter()
                .enumerate()
            {
                match self.build_e1_setup_item(&ue, x) {
                    Ok(item) => {
                        items.push(item);
                    }
                    Err(e) => {
                        debug!(self.logger, "Failed to build session setup item {:?}", e);
                    }
                };
            }

            if items.len() == 0 {
                break 'setup_stages;
            }

            // Send BearerContextSetup to CU-UP.
            let bearer_context_setup = self.build_bearer_context_setup(&ue, items);
            debug!(self.logger, "<< BearerContextSetupRequest");
            let response = match self
                .e1ap_request::<BearerContextSetupProcedure>(bearer_context_setup, self.logger)
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    debug!(self.logger, "Failed bearer context setup {:?}", e);
                    break 'setup_stages;
                }
            };
            debug!(self.logger, ">> BearerContextSetupResponse");
        }

        debug!(self.logger, "PduSessionResourceSetupResponse >> ");
        todo!()
        // PduSessionResourceSetupResponse {
        //     amf_ue_ngap_id: r.amf_ue_ngap_id,
        //     ran_ue_ngap_id: r.ran_ue_ngap_id,
        //     pdu_session_resource_setup_list_su_res,
        //     pdu_session_resource_failed_to_setup_list_su_res,
        //     criticality_diagnostics: None,
        // }
    }

    pub async fn pdu_session_resource_setup_stages<'b>(
        &self,
        r: &'b PduSessionResourceSetupRequest,
        successful: &mut Vec<Result<PduSessionResourceSetupItem, ngap::Cause>>,
    ) -> Result<()> {
        // Retrieve UE context by ran_ue_ngap_id.
        Ok(())
    }

    // pub async fn pdu_session_resource_setup(
    //     &self,
    //     r: PduSessionResourceSetupRequest,
    // ) -> PduSessionResourceSetupResponse {
    //     debug!(self.logger, "PduSessionResourceSetupRequest(Nas) << ");

    //     let (successful, unsuccessful) = match self.pdu_session_resource_setup_inner(&r).await {
    //         Ok(x) => x,
    //         Err(e) => {
    //             debug!(self.logger, "Failed resource setup - {}", e);
    //             (
    //                 Vec::new(),
    //                 r.pdu_session_resource_setup_list_su_req.0.iter().collect(),
    //             )
    //         }
    //     };

    //     // TODO: this is doable without cloning the pdu_session_resource_setup_request_transfer.

    //     let pdu_session_resource_setup_list_su_res = if successful.is_empty() {
    //         None
    //     } else {
    //         Some(PduSessionResourceSetupListSuRes(
    //             successful
    //                 .into_iter()
    //                 .map(|x| PduSessionResourceSetupItemSuRes {
    //                     pdu_session_id: x.pdu_session_id,
    //                     pdu_session_resource_setup_response_transfer: x
    //                         .pdu_session_resource_setup_request_transfer
    //                         .clone(),
    //                 })
    //                 .collect(),
    //         ))
    //     };

    //     let pdu_session_resource_failed_to_setup_list_su_res = if unsuccessful.is_empty() {
    //         None
    //     } else {
    //         Some(PduSessionResourceFailedToSetupListSuRes(
    //             unsuccessful
    //                 .into_iter()
    //                 .map(|x| PduSessionResourceFailedToSetupItemSuRes {
    //                     pdu_session_id: x.pdu_session_id,
    //                     pdu_session_resource_setup_unsuccessful_transfer: x
    //                         .pdu_session_resource_setup_request_transfer
    //                         .clone(),
    //                 })
    //                 .collect(),
    //         ))
    //     };

    //     debug!(self.logger, "PduSessionResourceSetupResponse >> ");
    //     PduSessionResourceSetupResponse {
    //         amf_ue_ngap_id: r.amf_ue_ngap_id,
    //         ran_ue_ngap_id: r.ran_ue_ngap_id,
    //         pdu_session_resource_setup_list_su_res,
    //         pdu_session_resource_failed_to_setup_list_su_res,
    //         criticality_diagnostics: None,
    //     }
    // }

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

        // Build E1 PduSessionResourceToSetupItems.
        let mut items = vec![];
        for x in r.pdu_session_resource_setup_list_su_req.0.iter() {
            match self.build_e1_setup_item(&ue, x) {
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
        let ue_context_setup_request =
            super::build_f1ap::build_ue_context_setup_request_from_pdu_session_setup(
                self.gnb_cu_cp,
                r,
                &ue,
                None,
            )?;
        self.log_message("<< UeContextSetupRequest");
        let ue_context_setup_response = self
            .f1ap_request::<UeContextSetupProcedure>(ue_context_setup_request, self.logger)
            .await?;
        self.log_message(">> UeContextSetupResponse");

        // Send BearerContextModification to CU-UP.
        let bearer_context_modification = self.build_bearer_context_modification(
            &ue,
            gnb_cu_up_ue_e1ap_id,
            &ue_context_setup_response,
        );
        self.log_message("<< BearerContextModificationRequest");
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
        let nas_messages: Vec<Vec<u8>> = successful
            .iter()
            .filter_map(|x| x.pdu_session_nas_pdu.as_ref().map(|x| x.0.clone()))
            .collect();

        // TS38.473, 8.3.1.2: "If the CellGroupConfig IE is included in the DU to CU RRC Information IE contained in the UE CONTEXT SETUP RESPONSE message,
        // the gNB-CU shall perform RRC Reconfiguration or RRC connection resume as described in TS 38.331 [8]. The CellGroupConfig IE shall
        // transparently be signaled to the UE as specified in TS 38.331 [8]."
        let cell_group_config = ue_context_setup_response
            .du_to_cu_rrc_information
            .cell_group_config
            .0;

        // Perform Rrc Reconfiguration including the Nas messages from earlier and the cell group config received from the DU.
        let rrc_transaction = self.new_rrc_transaction(&ue).await;
        let nas_messages = if nas_messages.is_empty() {
            None
        } else {
            Some(nas_messages)
        };
        let rrc_container =
            super::build_rrc::build_rrc_reconfiguration(3, nas_messages, cell_group_config)?;
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

    pub fn build_e1_setup_item(
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
            snssai: r.snssai.clone(),
            security_indication: SecurityIndication {
                integrity_protection_indication: IntegrityProtectionIndication::Preferred,
                confidentiality_protection_indication:
                    ConfidentialityProtectionIndication::Preferred,
                maximum_i_pdatarate: None,
            },
            pdu_session_resource_dl_ambr: None,
            // TODO: get transport information from the request
            ng_ul_up_tnl_information: UpTnlInformation::GtpTunnel(GtpTunnel {
                transport_layer_address: TransportLayerAddress(net::ip_bits_from_string(
                    "192.168.110.82",
                )?),
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
        _ue_context_setup_response: &f1ap::UeContextSetupResponse,
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
}
