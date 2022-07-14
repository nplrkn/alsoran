use crate::{procedures, Gnbcu, UeContext};
use anyhow::Result;
use async_trait::async_trait;
use f1ap::{GnbCuUeF1apId, GnbDuUeF1apId};
use net::{AperSerde, EventHandler, IndicationHandler, RequestError, RequestProvider, TnlaEvent};
use ngap::*;
use pdcp::PdcpPdu;
use rrc::*;
use slog::{debug, info, warn, Logger};

impl RequestProvider<NgSetupProcedure> for NgapHandler {}

#[derive(Clone)]
pub struct NgapHandler {
    gnbcu: Gnbcu,
}

impl NgapHandler {
    // So called because the the NgapGnb implements the Application trait.
    pub fn new_ngap_application(gnbcu: Gnbcu) -> NgapGnb<NgapHandler> {
        NgapGnb::new(NgapHandler { gnbcu })
    }
}
#[async_trait]
impl EventHandler for NgapHandler {
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        match event {
            TnlaEvent::Established(addr) => {
                info!(logger, "NGAP TNLA {} established to {}", tnla_id, addr);
                crate::procedures::ng_setup(&self.gnbcu, logger).await;
            }
            TnlaEvent::Terminated => warn!(logger, "NGAP TNLA {} closed", tnla_id),
        };
        self.gnbcu.connected_amf_change(logger).await;
    }
}

#[async_trait]
impl IndicationHandler<DownlinkNasTransportProcedure> for NgapHandler {
    async fn handle(&self, i: DownlinkNasTransport, logger: &Logger) {
        debug!(&logger, "DownlinkNasTransport(Nas) <<");
        // TODO - retrieve UE context by ran_ue_ngap_id.
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
        let rrc_container = f1ap::RrcContainer(PdcpPdu::encode(&rrc).into());
        debug!(&logger, "<< DlInformationTransfer(Nas)");
        self.gnbcu.send_rrc_to_ue(ue, rrc_container, logger).await;
    }
}

#[async_trait]
impl RequestProvider<InitialContextSetupProcedure> for NgapHandler {
    async fn request(
        &self,
        r: InitialContextSetupRequest,
        logger: &Logger,
    ) -> Result<InitialContextSetupResponse, RequestError<InitialContextSetupFailure>> {
        debug!(logger, "Initial Context Setup Procedure");
        procedures::initial_context_setup(&self.gnbcu, r, logger).await
    }
}

#[async_trait]
impl IndicationHandler<AmfStatusIndicationProcedure> for NgapHandler {
    async fn handle(&self, i: AmfStatusIndication, logger: &Logger) {
        debug!(logger, "<< Amf Status Indication");
        for guami_item in i.unavailable_guami_list.0 {
            info!(logger, "GUAMI {} now unavailable", guami_item.guami);
        }
    }
}
