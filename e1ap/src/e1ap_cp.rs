//! e1ap_cp - Collects together the procedures that are served by a GNB-CU-CP on the E1 reference point.

use super::top_pdu::*;
use crate::{E1apPdu, InitiatingMessage};
use asn1_per::*;
use async_trait::async_trait;
use net::{Application, EventHandler, TnlaEvent};
use slog::{error, Logger};

#[derive(Clone)]
pub struct E1apCp<T>(pub T)
where
    T: EventHandler;

impl<T: EventHandler> E1apCp<T> {
    pub fn new(inner: T) -> Self {
        E1apCp(inner)
    }
}

#[async_trait]
impl<T> EventHandler for E1apCp<T>
where
    T: EventHandler,
{
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        self.0.handle_event(event, tnla_id, logger).await;
    }
}

impl<T> Application for E1apCp<T> where
    T: RequestProvider<GnbCuUpE1SetupProcedure> + EventHandler + Clone
{
}

#[async_trait]
impl<T> InterfaceProvider for E1apCp<T>
where
    T: Send + Sync + EventHandler + RequestProvider<GnbCuUpE1SetupProcedure>, // Todo - add all other procedures
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
            InitiatingMessage::GnbCuUpE1SetupRequest(req) => {
                GnbCuUpE1SetupProcedure::call_provider(&self.0, req, logger).await
            }
            _ => todo!(),
        }
    }
}
