use super::top_pdu::*;
use crate::{InitiatingMessage, NgapPdu, SuccessfulOutcome};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use net::{Application, EventHandler, InterfaceProvider, Procedure, RequestProvider, TnlaEvent};
use slog::Logger;

#[derive(Clone)]
pub struct NgapGnb<T>(pub T)
where
    T: RequestProvider<NgSetupProcedure>; // TODO

#[async_trait]
impl<T> EventHandler for NgapGnb<T>
where
    T: RequestProvider<NgSetupProcedure> + EventHandler,
{
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        self.0.handle_event(event, tnla_id, logger).await;
    }
}

impl<T> Application for NgapGnb<T> where T: RequestProvider<NgSetupProcedure> + EventHandler + Clone {}

#[async_trait]
impl<T> InterfaceProvider for NgapGnb<T>
where
    T: Send + Sync + RequestProvider<NgSetupProcedure>,
{
    type TopPdu = NgapPdu;
    async fn route_request(&self, p: NgapPdu, logger: &Logger) -> Result<NgapPdu> {
        let initiating_message = match p {
            NgapPdu::InitiatingMessage(m) => m,
            _ => return Err(anyhow!("Not a request!")),
        };
        match initiating_message {
            InitiatingMessage::NgSetupRequest(req) => {
                NgSetupProcedure::call_provider(&self.0, req, logger)
                    .await
                    .ok_or(anyhow!("No response received"))
            }
            _ => todo!(),
        }
    }
}

// async fn route_request<T>(&provider: T, req: Message, logger: &Logger) -> Result<NgapPdu> {
//     match <T as RequestProvider<NgSetupProcedure>>::request(provider, req, logger).await {
//         Ok(x) => Ok(NgapPdu::SuccessfulOutcome(NgSetupRequest(x))),
//         Err(_) => todo!(),
//     }
// }
