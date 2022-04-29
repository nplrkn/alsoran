use super::Gnbcu;
use anyhow::Result;
use async_trait::async_trait;
use bitvec::prelude::*;
use f1ap::*;
use net::{Application, Message, TnlaEvent};
use slog::{trace, warn, Logger};
use xxap_transaction::*;

#[derive(Clone)]
pub struct F1apHandler {
    gnbcu: Gnbcu,
}

impl F1apHandler {
    pub fn new(gnbcu: Gnbcu) -> Self {
        Self { gnbcu }
    }
}

#[async_trait]
impl RequestProvider<F1SetupProcedure> for F1apHandler {
    async fn request(
        &self,
        r: F1SetupRequest,
        logger: &Logger,
    ) -> Result<F1SetupResponse, RequestError<F1SetupFailure>> {
        todo!()
    }
}

#[async_trait]
impl Application for F1apHandler {
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        match event {
            TnlaEvent::Established => trace!(logger, "F1AP TNLA {} established", tnla_id),
            TnlaEvent::Terminated => warn!(logger, "F1AP TNLA {} closed", tnla_id),
        };
    }

    async fn handle_request(
        &self,
        message: Message,
        _tnla_id: u32,
        logger: &Logger,
    ) -> Result<Message> {
        let pdu = F1apPdu::from_bytes(&message).unwrap(); // TODO
        trace!(logger, "f1ap_pdu: {:?}", pdu);
        todo!()
        // if let Some(response) = match pdu {
        //     F1apPdu::InitiatingMessage(InitiatingMessage::F1SetupRequest(x)) => {
        //         Some(self.f1_setup(x, logger).await)
        //     }
        //     x => {
        //         warn!(logger, "Unexpected or unhandled PDU {:?}", x);
        //         None
        //     }
        // } {
        //     let response = F1apPdu::SuccessfulOutcome(SuccessfulOutcome::F1SetupResponse(response));
        //     self.gnbcu
        //         .f1ap
        //         .transport_provider
        //         .send_pdu(response, logger) // include tnla id in future
        //         .await
        //         .unwrap_or_else(|e| warn!(self.logger, "Failed to send response {:?}", e));
        // }
    }
}

// impl<
//         N: TransportProvider,
//         F: TransportProvider,
//         C: Api<ClientContext> + Send + Sync + Clone + 'static,
//     > Gnbcu<N, F, C>
// {
//     async fn f1_setup(&self, message: F1SetupRequest, _logger: &Logger) -> F1SetupResponse {
//         F1SetupResponse {
//             transaction_id: message.transaction_id,
//             gnb_cu_rrc_version: RrcVersion {
//                 latest_rrc_version: bitvec![Msb0, u8;0, 0, 0],
//             },
//             gnb_cu_name: None,
//             cells_to_be_activated_list: None,
//             transport_layer_address_info: None,
//             ul_bh_non_up_traffic_mapping: None,
//             bap_address: None,
//             extended_gnb_du_name: None,
//         }
//     }
// }
