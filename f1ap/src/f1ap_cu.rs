//! f1ap_cu - Collects together the procedures that are served by a GNB-CU on the F1 reference point.

use super::top_pdu::*;
use crate::{F1apPdu, InitiatingMessage};
use asn1_per::*;
use async_trait::async_trait;
use net::{Application, EventHandler, TnlaEvent};
use slog::{error, Logger};

#[derive(Clone)]
pub struct F1apCu<T>(pub T)
where
    T: EventHandler;

impl<T: EventHandler> F1apCu<T> {
    pub fn new(inner: T) -> Self {
        F1apCu(inner)
    }
}

#[async_trait]
impl<T> EventHandler for F1apCu<T>
where
    T: EventHandler,
{
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        self.0.handle_event(event, tnla_id, logger).await;
    }
}

impl<T> Application for F1apCu<T> where
    T: RequestProvider<F1SetupProcedure>
        + RequestProvider<GnbDuConfigurationUpdateProcedure>
        + EventHandler
        + Clone
        + IndicationHandler<InitialUlRrcMessageTransferProcedure>
        + IndicationHandler<UlRrcMessageTransferProcedure>
{
}

#[async_trait]
impl<T> InterfaceProvider for F1apCu<T>
where
    T: Send
        + Sync
        + EventHandler
        + RequestProvider<F1SetupProcedure>
        + RequestProvider<GnbDuConfigurationUpdateProcedure>
        + IndicationHandler<InitialUlRrcMessageTransferProcedure>
        + IndicationHandler<UlRrcMessageTransferProcedure>,
    // Todo - add all other procedures
{
    type TopPdu = F1apPdu;
    async fn route_request(&self, p: F1apPdu, logger: &Logger) -> Option<ResponseAction<F1apPdu>> {
        let initiating_message = match p {
            F1apPdu::InitiatingMessage(m) => m,
            x => {
                error!(logger, "Not a request! {:?}", x);
                return None;
            }
        };
        match initiating_message {
            InitiatingMessage::F1SetupRequest(req) => {
                F1SetupProcedure::call_provider(&self.0, req, logger).await
            }
            InitiatingMessage::InitialUlRrcMessageTransfer(req) => {
                InitialUlRrcMessageTransferProcedure::call_provider(&self.0, req, logger).await;
                None
            }
            InitiatingMessage::UlRrcMessageTransfer(req) => {
                UlRrcMessageTransferProcedure::call_provider(&self.0, req, logger).await;
                None
            }
            InitiatingMessage::GnbDuConfigurationUpdate(req) => {
                GnbDuConfigurationUpdateProcedure::call_provider(&self.0, req, logger).await
            }
            _ => todo!(),
        }
    }
}
