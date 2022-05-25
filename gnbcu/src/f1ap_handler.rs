use super::{Gnbcu, UeContext, RrcHandler};
use async_trait::async_trait;
use bitvec::prelude::*;
use f1ap::*;
use net::{EventHandler, RequestError, RequestProvider, TnlaEvent};
use slog::{info, warn, Logger};

#[derive(Clone)]
pub struct F1apHandler {
    pub gnbcu: Gnbcu,
    pub rrc_handler: RrcHandler,
}

pub fn new(gnbcu: Gnbcu, rrc_handler: RrcHandler) -> F1apCu<F1apHandler> {
    F1apCu(F1apHandler { gnbcu, rrc_handler })
}



#[async_trait]
impl RequestProvider<F1SetupProcedure> for F1apHandler {
    async fn request(
        &self,
        r: F1SetupRequest,
        logger: &Logger,
    ) -> Result<F1SetupResponse, RequestError<F1SetupFailure>> {
        info!(logger, "Got F1 setup - send response");
        Ok(F1SetupResponse {
            transaction_id: r.transaction_id,
            gnb_cu_rrc_version: RrcVersion {
                latest_rrc_version: bitvec![Msb0, u8;0, 0, 0],
            },
            gnb_cu_name: None,
            cells_to_be_activated_list: None,
            transport_layer_address_info: None,
            ul_bh_non_up_traffic_mapping: None,
            bap_address: None,
            extended_gnb_du_name: None,
        })
    }
}

#[async_trait]
impl RequestProvider<InitialUlRrcMessageTransferProcedure> for F1apHandler {
    async fn request(
        &self,
        r: InitialUlRrcMessageTransfer,
        logger: &Logger,
    ) -> Result<(), RequestError<()>> {
        info!(logger, "Got InitialUlRrcMessageTransfer");

        // TODO - "If the DU to CU RRC Container IE is not included in the INITIAL UL RRC MESSAGE TRANSFER, 
        // the gNB-CU should reject the UE under the assumption that the gNB-DU is not able to serve such UE."

        // TODO - "If the RRC-Container-RRCSetupComplete IE is included in the INITIAL UL RRC MESSAGE TRANSFER, 
        // the gNB-CU shall take it into account as specified in TS 38.401 [4]."

        let ue_context = UeContext {
            gnb_du_ue_f1ap_id: r.gnb_du_ue_f1ap_id
        };

        self.rrc_handler.dispatch(ue_context, &r.rrc_container.0);
        Ok(())
    }
}

#[async_trait]
impl EventHandler for F1apHandler {
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        match event {
            TnlaEvent::Established => info!(logger, "F1AP TNLA {} established", tnla_id),
            TnlaEvent::Terminated => warn!(logger, "F1AP TNLA {} closed", tnla_id),
        };
    }
}
