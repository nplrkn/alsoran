//! f1ap - F1AP entry points into the GNB-CU

use super::Gnbcu;
use super::RrcHandler;
use crate::workflows;
use async_trait::async_trait;
use bitvec::prelude::*;
use f1ap::*;
use net::{EventHandler, IndicationHandler, RequestError, RequestProvider, TnlaEvent};
use pdcp::PdcpPdu;
use slog::{debug, info, warn, Logger};

#[derive(Clone)]
pub struct F1apHandler<G: Gnbcu> {
    gnbcu: G,
    rrc_handler: RrcHandler<G>,
}

impl<G: Gnbcu> F1apHandler<G> {
    pub fn new_f1ap_application(gnbcu: G, rrc_handler: RrcHandler<G>) -> F1apCu<F1apHandler<G>> {
        F1apCu::new(F1apHandler {
            gnbcu: gnbcu,
            rrc_handler,
        })
    }
}

#[async_trait]
impl<G: Gnbcu> RequestProvider<F1SetupProcedure> for F1apHandler<G> {
    async fn request(
        &self,
        r: F1SetupRequest,
        logger: &Logger,
    ) -> Result<F1SetupResponse, RequestError<F1SetupFailure>> {
        debug!(logger, ">> F1SetupRequest");
        info!(logger, "F1AP interface initialized with {:?}", r.gnb_du_id);
        debug!(logger, "<< F1SetupResponse");
        Ok(F1SetupResponse {
            transaction_id: r.transaction_id,
            gnb_cu_rrc_version: RrcVersion {
                latest_rrc_version: bitvec![u8, Msb0;0, 0, 0],
            },
            gnb_cu_name: self.gnbcu.config().clone().name.map(|x| GnbCuName(x)),
            cells_to_be_activated_list: None,
            transport_layer_address_info: None,
            ul_bh_non_up_traffic_mapping: None,
            bap_address: None,
            extended_gnb_du_name: None,
        })
    }
}

#[async_trait]
impl<G: Gnbcu> IndicationHandler<InitialUlRrcMessageTransferProcedure> for F1apHandler<G> {
    async fn handle(&self, r: InitialUlRrcMessageTransfer, logger: &Logger) {
        debug!(logger, ">> InitialUlRrcMessageTransfer");
        if let Err(e) = workflows::initial_access(&self.gnbcu, r, logger).await {
            debug!(logger, "Inital access procedure failed - {:?}", e);
        }
    }
}

#[async_trait]
impl<G: Gnbcu> IndicationHandler<UlRrcMessageTransferProcedure> for F1apHandler<G> {
    async fn handle(&self, r: UlRrcMessageTransfer, logger: &Logger) {
        debug!(logger, ">> UlRrcMessageTransfer");

        // TODO: "If the UL RRC MESSAGE TRANSFER message contains the New gNB-DU UE F1AP ID IE, the gNB-CU shall,
        // if supported, replace the value received in the gNB-DU UE F1AP ID IE by the value of the New gNB-DU UE F1AP ID
        // and use it for further signalling."

        let pdcp_pdu = PdcpPdu(r.rrc_container.0);

        let rrc_message_bytes = match pdcp_pdu.view_inner() {
            Ok(x) => x,
            Err(e) => {
                warn!(logger, "Invalid PDCP PDU - {:?}", e);
                return;
            }
        };

        self.rrc_handler
            .dispatch_dcch(r.gnb_cu_ue_f1ap_id.0, rrc_message_bytes, logger)
            .await;
    }
}

#[async_trait]
impl<G: Gnbcu> EventHandler for F1apHandler<G> {
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        match event {
            TnlaEvent::Established(addr) => {
                info!(logger, "F1AP TNLA {} established from {}", tnla_id, addr)
            }
            TnlaEvent::Terminated => warn!(logger, "F1AP TNLA {} closed", tnla_id),
        };
    }
}
