use super::Gnbcu;
use crate::ue_context::UeContext;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bitvec::prelude::*;
use f1ap::{GnbCuUeF1apId, GnbDuUeF1apId, UeContextSetupProcedure, UeContextSetupRequest};
use net::{
    AperSerde, EventHandler, IndicationHandler, RequestError, RequestProvider, Stack, TnlaEvent,
};
use ngap::*;
use pdcp::PdcpPdu;
use rrc::{
    CriticalExtensions4, DedicatedNasMessage, DlDcchMessage, DlDcchMessageType,
    DlInformationTransfer, DlInformationTransferIEs, RrcTransactionIdentifier, C1_2,
};
use slog::{debug, info, warn, Logger};

impl RequestProvider<NgSetupProcedure> for Handler {}

pub fn new(gnbcu: Gnbcu) -> NgapGnb<Handler> {
    NgapGnb(Handler { gnbcu })
}
#[derive(Clone)]
pub struct Handler {
    pub gnbcu: Gnbcu,
}

#[async_trait]
impl EventHandler for Handler {
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        match event {
            TnlaEvent::Established => {
                info!(logger, "NGAP TNLA {} established", tnla_id);

                // This uses the default expected values of free5GC.
                let ng_setup_request = NgSetupRequest {
                    global_ran_node_id: GlobalRanNodeId::GlobalGnbId(GlobalGnbId {
                        plmn_identity: PlmnIdentity(vec![0x2, 0xf8, 0x39]),
                        gnb_id: GnbId::GnbId(bitvec![Msb0,u8; 1; 22]),
                    }),
                    ran_node_name: None,
                    supported_ta_list: SupportedTaList(vec![SupportedTaItem {
                        tac: Tac(vec![0x0, 0x0, 0x1]),
                        broadcast_plmn_list: BroadcastPlmnList(vec![BroadcastPlmnItem {
                            plmn_identity: PlmnIdentity(vec![0x2, 0xf8, 0x39]),
                            tai_slice_support_list: SliceSupportList(vec![SliceSupportItem {
                                s_nssai: SNssai {
                                    sst: Sst(vec![0x01]),
                                    sd: Some(Sd(vec![0x1, 0x2, 0x3])),
                                },
                            }]),
                        }]),
                    }]),
                    default_paging_drx: PagingDrx::V128,
                    ue_retention_information: None,
                    nb_iot_default_paging_drx: None,
                    extended_ran_node_name: None,
                };
                let ng_setup_provider = &self.gnbcu.ngap;
                info!(logger, "Send NG Setup");
                match <Stack as RequestProvider<NgSetupProcedure>>::request(
                    ng_setup_provider,
                    ng_setup_request,
                    logger,
                )
                .await
                {
                    Ok(_response) => info!(logger, "Successful NG Setup"),
                    Err(e) => warn!(logger, "NG Setup failed - {:?}", e),
                }
            }
            TnlaEvent::Terminated => warn!(logger, "NGAP TNLA {} closed", tnla_id),
        };
        self.gnbcu.connected_amf_change(logger).await;
    }
}

#[async_trait]
impl IndicationHandler<DownlinkNasTransportProcedure> for Handler {
    async fn handle(&self, i: DownlinkNasTransport, logger: &Logger) {
        debug!(
            &logger,
            "Got Downlink Nas Transport - send RRC to UE via DU"
        );
        // To do - retrieve UE context by ran_ue_ngap_id.
        let ue = UeContext {
            gnb_du_ue_f1ap_id: GnbDuUeF1apId(1),
            gnb_cu_ue_f1ap_id: GnbCuUeF1apId(1),
        };

        let rrc = match (DlDcchMessage {
            message: DlDcchMessageType::C1(C1_2::DlInformationTransfer(DlInformationTransfer {
                rrc_transaction_identifier: RrcTransactionIdentifier(2),
                critical_extensions: CriticalExtensions4::DlInformationTransfer(
                    DlInformationTransferIEs {
                        dedicated_nas_message: Some(DedicatedNasMessage(i.nas_pdu.0)),
                        late_non_critical_extension: None,
                        non_critical_extension: None,
                    },
                ),
            })),
        }
        .into_bytes())
        {
            Ok(x) => x,
            Err(e) => {
                warn!(
                    logger,
                    "Failed to encode Rrc DlInformationTransfer - {:?}", e
                );
                return;
            }
        };
        let rrc_container = f1ap::RrcContainer(PdcpPdu::encode(&rrc).bytes());
        self.gnbcu.send_rrc_to_ue(ue, rrc_container, logger).await;
    }
}

fn make_rrc_container_for_nas(
    _ue: &UeContext,
    nas: Vec<u8>,
    _logger: &Logger,
) -> Result<f1ap::RrcContainer> {
    match (DlDcchMessage {
        message: DlDcchMessageType::C1(C1_2::DlInformationTransfer(DlInformationTransfer {
            rrc_transaction_identifier: RrcTransactionIdentifier(2),
            critical_extensions: CriticalExtensions4::DlInformationTransfer(
                DlInformationTransferIEs {
                    dedicated_nas_message: Some(DedicatedNasMessage(nas)),
                    late_non_critical_extension: None,
                    non_critical_extension: None,
                },
            ),
        })),
    }
    .into_bytes())
    {
        Ok(x) => Ok(f1ap::RrcContainer(PdcpPdu::encode(&x).bytes())),
        Err(e) => Err(anyhow!(format!(
            "Failed to encode Rrc DlInformationTransfer- {:?}",
            e
        ))),
    }
}

#[async_trait]
impl RequestProvider<InitialContextSetupProcedure> for Handler {
    async fn request(
        &self,
        r: InitialContextSetupRequest,
        logger: &Logger,
    ) -> Result<InitialContextSetupResponse, RequestError<InitialContextSetupFailure>> {
        debug!(
            logger,
            "Received NGAP InitialContextSetupRequest, send F1AP UeContextSetupRequest"
        );

        // To do - retrieve UE context by ran_ue_ngap_id.
        let ue = UeContext {
            gnb_du_ue_f1ap_id: GnbDuUeF1apId(1),
            gnb_cu_ue_f1ap_id: GnbCuUeF1apId(1),
        };

        // Take the NAS message out and wrap it in an RRC container.
        let rrc_container = match r.nas_pdu {
            None => None,
            Some(x) => Some(make_rrc_container_for_nas(&ue, x.0, logger)?),
        };

        // TODO: derive and use frunk for the common ngap / f1ap structures seen here.
        let ue_context_setup_request = UeContextSetupRequest {
            gnb_cu_ue_f1ap_id: GnbCuUeF1apId(1),
            gnb_du_ue_f1ap_id: Some(GnbDuUeF1apId(1)),
            sp_cell_id: f1ap::NrCgi {
                plmn_identity: f1ap::PlmnIdentity(vec![0, 1, 2]),
                nr_cell_identity: f1ap::NrCellIdentity(bitvec![Msb0,u8;0;36]),
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
            drbs_to_be_setup_list: None,
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
        };

        match <Stack as RequestProvider<UeContextSetupProcedure>>::request(
            &self.gnbcu.f1ap,
            ue_context_setup_request,
            &logger,
        )
        .await
        {
            Ok(_x) => todo!(),
            Err(_) => todo!(),
        }
    }
}
