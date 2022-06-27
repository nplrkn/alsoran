use crate::{procedures, Gnbcu, UeContext};
use anyhow::Result;
use async_trait::async_trait;
use f1ap::{GnbCuUeF1apId, GnbDuUeF1apId};
use net::{AperSerde, EventHandler, IndicationHandler, RequestError, RequestProvider, TnlaEvent};
use ngap::*;
use pdcp::PdcpPdu;
use rrc::*;
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
                crate::procedures::ng_setup(&self.gnbcu, logger).await;
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

#[async_trait]
impl RequestProvider<InitialContextSetupProcedure> for Handler {
    async fn request(
        &self,
        r: InitialContextSetupRequest,
        logger: &Logger,
    ) -> Result<InitialContextSetupResponse, RequestError<InitialContextSetupFailure>> {
        debug!(logger, "Initial Context Setup Procedure");
        procedures::initial_context_setup(&self.gnbcu, r, logger).await
    }
}
