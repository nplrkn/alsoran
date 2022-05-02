use super::procedures::*;
use crate::{InitiatingMessage, NgapPdu, SuccessfulOutcome};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use net::{Application, EventHandler, InterfaceProvider, RequestProvider, TnlaEvent};
use slog::Logger;

#[derive(Clone)]
pub struct NgapGnb<T>(pub T)
where
    T: RequestProvider<NgSetupRequestProcedure>; // TODO

#[async_trait]
impl<T> EventHandler for NgapGnb<T>
where
    T: RequestProvider<NgSetupRequestProcedure> + EventHandler,
{
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        self.0.handle_event(event, tnla_id, logger).await;
    }
}

impl<T> Application for NgapGnb<T> where
    T: RequestProvider<NgSetupRequestProcedure> + EventHandler + Clone
{
}

#[async_trait]
impl<T> InterfaceProvider for NgapGnb<T>
where
    T: Send + Sync + RequestProvider<NgSetupRequestProcedure>,
{
    type TopPdu = NgapPdu;
    async fn route_request(&self, p: NgapPdu, logger: &Logger) -> Result<NgapPdu> {
        let initiating_message = match p {
            NgapPdu::InitiatingMessage(m) => m,
            _ => return Err(anyhow!("Not a request!")),
        };
        match initiating_message {
            InitiatingMessage::NgSetupRequest(req) => {
                match <T as RequestProvider<NgSetupRequestProcedure>>::request(&self.0, req, logger)
                    .await
                {
                    Ok(x) => Ok(NgapPdu::SuccessfulOutcome(
                        SuccessfulOutcome::NgSetupResponse(x),
                    )),
                    Err(_) => todo!(),
                }
            }
            _ => todo!(),
        }
    }
}
