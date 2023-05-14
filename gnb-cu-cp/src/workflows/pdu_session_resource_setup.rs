//! pdu_session_resource_setup - AMF orders setup of PDU sessions and DRBs

use super::{build_e1ap, GnbCuCp, Workflow};
use crate::datastore::UeState;
use anyhow::{anyhow, bail, ensure, Result};
use e1ap::*;
use f1ap::{
    CellGroupConfig, DlUpTnlInformationToBeSetupItem, DrbsToBeSetupList, UeContextSetupProcedure,
};
use net::SerDes;
use ngap::{
    AssociatedQosFlowItem, AssociatedQosFlowList, PduSessionResourceFailedToSetupItemSuRes,
    PduSessionResourceFailedToSetupListSuRes, PduSessionResourceSetupItemSuReq,
    PduSessionResourceSetupItemSuRes, PduSessionResourceSetupListSuRes,
    PduSessionResourceSetupRequest, PduSessionResourceSetupResponse,
    PduSessionResourceSetupResponseTransfer, QosFlowPerTnlInformation, UpTransportLayerInformation,
};
use slog::{debug, warn, Logger};
use xxap::*;

type Stage1 = ngap::PduSessionResourceSetupItemSuReq;
type Stage2 = (Stage1, e1ap::PduSessionResourceSetupItem);
type Stage3 = (Stage2, f1ap::DrbsSetupItem);
type Stage4 = (Stage3, e1ap::PduSessionResourceModifiedItem);

impl<'a, G: GnbCuCp> Workflow<'a, G> {
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
        let requested_session_ids: Vec<PduSessionId> = r
            .pdu_session_resource_setup_list_su_req
            .0
            .iter()
            .map(|item| item.pdu_session_id)
            .collect();

        // Go through all the stages of session resource setup.  If all goes well, the
        // successfully set up sessions will pop out the other side.
        let ok_sessions = self
            .pdu_session_resource_setup_stages(r)
            .await
            .unwrap_or_else(|e| {
                warn!(self.logger, "Error processing session setup - {}", e);
                vec![]
            });

        let failed_sessions: Vec<PduSessionResourceFailedToSetupItemSuRes> = requested_session_ids
            .into_iter()
            .filter(|x| ok_sessions.iter().all(|item| item.pdu_session_id.0 != x.0))
            .map(|x| PduSessionResourceFailedToSetupItemSuRes {
                pdu_session_id: x,
                pdu_session_resource_setup_unsuccessful_transfer: vec![],
            })
            .collect();

        let pdu_session_resource_setup_list_su_res = if ok_sessions.is_empty() {
            None
        } else {
            Some(PduSessionResourceSetupListSuRes(ok_sessions))
        };

        let pdu_session_resource_failed_to_setup_list_su_res = if failed_sessions.is_empty() {
            None
        } else {
            Some(PduSessionResourceFailedToSetupListSuRes(failed_sessions))
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
        r: PduSessionResourceSetupRequest,
    ) -> Result<Vec<PduSessionResourceSetupItemSuRes>> {
        debug!(self.logger, "Retrieve UE {:#010x}", r.ran_ue_ngap_id.0);
        let mut ue = self.retrieve(&r.ran_ue_ngap_id.0).await?;

        // E1 BearerContextSetupRequest.
        let sessions = self
            .pdu_session_resource_setup_stage_1(&mut ue, r.pdu_session_resource_setup_list_su_req.0)
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

    async fn pdu_session_resource_setup_stage_1(
        &self,
        ue: &mut UeState,
        sessions: Vec<Stage1>,
    ) -> Vec<Stage2> {
        // Build E1 PduSessionResourceToSetupItems.
        let mut items = vec![];
        for session in &sessions {
            match build_e1ap::build_e1_setup_item(&ue, &session) {
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

        let items = self.perform_bearer_context_setup(ue, items).await;

        keep_matching_items(sessions, items, self.logger)
    }

    async fn pdu_session_resource_setup_stage_2(
        &self,
        ue: &UeState,
        sessions: Vec<Stage2>,
    ) -> Result<(Vec<Stage3>, CellGroupConfig)> {
        let mut items = vec![];
        for session in &sessions {
            let (
                PduSessionResourceSetupItemSuReq {
                    pdu_session_id,
                    snssai,
                    ..
                },
                PduSessionResourceSetupItem {
                    ng_dl_up_tnl_information: UpTnlInformation::GtpTunnel(gtp_tunnel),
                    ..
                },
            ) = session;

            // Pass the transport address of the CU-UP to the DU.
            let snssai: xxap::Snssai = snssai.clone().into();
            match super::build_f1ap::build_drb_to_be_setup_item(
                f1ap::DrbId(pdu_session_id.0),
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
        let drbs_setup_list = match ue_context_setup_response.drbs_setup_list {
            Some(x) => x.0,
            _ => {
                bail!("UeContextSetupResponse without DRB setup list");
            }
        };

        let successful_sessions = keep_matching_items(sessions, drbs_setup_list, self.logger);

        Ok((successful_sessions, cell_group_config))
    }

    async fn perform_bearer_context_setup(
        &self,
        ue: &mut UeState,
        items: Vec<PduSessionResourceToSetupItem>,
    ) -> Vec<PduSessionResourceSetupItem> {
        // Send BearerContextSetup to CU-UP.
        let bearer_context_setup = build_e1ap::build_bearer_context_setup(
            &ue,
            PlmnIdentity(self.config().plmn.clone()),
            items,
        );

        debug!(self.logger, "<< BearerContextSetupRequest");
        match self
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
                debug!(self.logger, ">> BearerContextSetupResponse");

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
        }
    }

    async fn perform_bearer_context_modification(
        &self,
        ue: &UeState,
        items: Vec<PduSessionResourceToModifyItem>,
    ) -> Vec<PduSessionResourceModifiedItem> {
        let Some(gnb_cu_up_ue_e1ap_id) = ue.gnb_cu_up_ue_e1ap_id else {
            warn!(self.logger, "No E1AP ID on UE");
            return vec![]
        };

        let bearer_context_modification =
            build_e1ap::build_bearer_context_modification(&ue, gnb_cu_up_ue_e1ap_id, items);
        debug!(self.logger, "<< BearerContextSetupRequest");
        let resource_modify_items = match self
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
        resource_modify_items
    }

    async fn pdu_session_resource_setup_stage_3(
        &self,
        ue: &UeState,
        sessions: Vec<Stage3>,
    ) -> Vec<Stage4> {
        let e1_items = build_e1_modify_items(&sessions, self.logger);
        if e1_items.is_empty() {
            warn!(self.logger, "No sessions left");
            return vec![];
        }

        let f1_items = self.perform_bearer_context_modification(ue, e1_items).await;

        // Rebuild the session results vec, adding in the new info from the UP.
        keep_matching_items(sessions, f1_items, self.logger)
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
            .filter_map(|x| x.0 .0 .0.pdu_session_nas_pdu.as_ref().map(|x| x.0.clone()))
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
            let pdu_session_id = PduSessionId(session.id());
            let UpTnlInformation::GtpTunnel(gtp_tunnel) = session.0 .0 .1.ng_dl_up_tnl_information;
            let new_session = PduSessionResourceSetupItemSuRes {
                pdu_session_id,
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
}

trait HasId {
    fn id(&self) -> u8;
}

// Outputs a list of pairs of elements with matching ids from the two input lists.
// (Not an efficient algorithm for long lists.)
fn keep_matching_items<T1: HasId, T2: HasId>(
    mut sessions: Vec<T1>,
    mut items: Vec<T2>,
    logger: &Logger,
) -> Vec<(T1, T2)> {
    let mut new_sessions: Vec<(T1, T2)> = vec![];
    for session in sessions.drain(..) {
        let pdu_session_id = session.id();
        let index = items.iter().position(|item| item.id() == pdu_session_id);
        match index {
            Some(index) => new_sessions.push((session, items.swap_remove(index))),
            None => {
                warn!(logger, "Session {} not found", pdu_session_id);
            }
        };
    }
    new_sessions
}

impl HasId for Stage1 {
    fn id(&self) -> u8 {
        self.pdu_session_id.0
    }
}
impl HasId for e1ap::PduSessionResourceSetupItem {
    fn id(&self) -> u8 {
        self.pdu_session_id.0
    }
}
impl HasId for f1ap::DrbsSetupItem {
    fn id(&self) -> u8 {
        self.drb_id.0
    }
}
impl HasId for e1ap::PduSessionResourceModifiedItem {
    fn id(&self) -> u8 {
        self.pdu_session_id.0
    }
}
impl HasId for Stage2 {
    fn id(&self) -> u8 {
        self.0.id()
    }
}
impl HasId for Stage3 {
    fn id(&self) -> u8 {
        self.0.id()
    }
}
impl HasId for Stage4 {
    fn id(&self) -> u8 {
        self.0.id()
    }
}

fn build_e1_modify_items(
    sessions: &Vec<Stage3>,
    logger: &Logger,
) -> Vec<PduSessionResourceToModifyItem> {
    let mut items = vec![];
    for session in sessions {
        match session
            // Get the tunnel information returned by the DU...
            .1
            .dl_up_tnl_information_to_be_setup_list
            .0
            .first()
            .ok_or_else(|| anyhow!("No GTP tunnel information from DU"))
            // ...reformulate to give it to the CU-UP
            .and_then(
                |DlUpTnlInformationToBeSetupItem {
                     dl_up_tnl_information:
                         f1ap::UpTransportLayerInformation::GtpTunnel(gtp_tunnel),
                 }| {
                    build_e1ap::build_e1_modify_item(
                        PduSessionId(session.id()),
                        gtp_tunnel.clone(),
                    )
                },
            ) {
            // ...and store in the list
            Ok(item) => items.push(item),
            Err(e) => {
                warn!(logger, "Build E1 setup item failed {:?}", e);
            }
        };
    }
    items
}
