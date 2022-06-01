use super::Gnbcu;
use crate::ue_context::UeContext;
use async_trait::async_trait;
use bitvec::prelude::*;
use f1ap::{GnbCuUeF1apId, GnbDuUeF1apId};
use net::{AperSerde, EventHandler, IndicationHandler, RequestProvider, Stack, TnlaEvent};
use ngap::*;
use pdcp::PdcpPdu;
use rrc::{
    CriticalExtensions4, DedicatedNasMessage, DlDcchMessage, DlDcchMessageType,
    DlInformationTransfer, DlInformationTransferIEs, RrcTransactionIdentifier, C1_2,
};
use slog::{info, warn, Logger};

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
