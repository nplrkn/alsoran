//! e1ap_up - Collects together the procedures that are served by a GNB-CU-UP on the E1 reference point.

use super::top_pdu::*;
use crate::E1apPdu;
use asn1_per::*;
use async_trait::async_trait;
use net::{Application, EventHandler, TnlaEvent};
use slog::{error, Logger};

#[derive(Clone)]
pub struct E1apUp<T>(pub T)
where
    T: EventHandler;

impl<T: EventHandler> E1apUp<T> {
    pub fn new(inner: T) -> Self {
        E1apUp(inner)
    }
}

#[async_trait]
impl<T> EventHandler for E1apUp<T>
where
    T: EventHandler,
{
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        self.0.handle_event(event, tnla_id, logger).await;
    }
}

impl<T> Application for E1apUp<T> where
    T: RequestProvider<BearerContextSetupProcedure>
        + RequestProvider<BearerContextModificationProcedure>
        + RequestProvider<BearerContextReleaseProcedure>
        + RequestProvider<GnbCuCpConfigurationUpdateProcedure>
        + EventHandler
        + Clone
{
}

#[async_trait]
impl<T> InterfaceProvider for E1apUp<T>
where
    T: Send
        + Sync
        + EventHandler
        + RequestProvider<GnbCuCpConfigurationUpdateProcedure>
        + RequestProvider<BearerContextSetupProcedure>
        + RequestProvider<BearerContextModificationProcedure>
        + RequestProvider<BearerContextReleaseProcedure>,
    // Todo - add all other procedures
{
    type TopPdu = E1apPdu;
    async fn route_request(&self, p: E1apPdu, logger: &Logger) -> Option<ResponseAction<E1apPdu>> {
        let initiating_message = match p {
            E1apPdu::InitiatingMessage(m) => m,
            x => {
                error!(logger, "Not a request! {:?}", x);
                return None;
            }
        };
        match initiating_message {
            InitiatingMessage::BearerContextSetupRequest(req) => {
                BearerContextSetupProcedure::call_provider(&self.0, req, logger).await
            }
            InitiatingMessage::BearerContextModificationRequest(req) => {
                BearerContextModificationProcedure::call_provider(&self.0, req, logger).await
            }
            InitiatingMessage::GnbCuCpConfigurationUpdate(req) => {
                GnbCuCpConfigurationUpdateProcedure::call_provider(&self.0, req, logger).await
            }
            InitiatingMessage::BearerContextReleaseCommand(req) => {
                BearerContextReleaseProcedure::call_provider(&self.0, req, logger).await
            }
            e => panic!("Missing implementation for {:?}", e),
        }
    }
}
