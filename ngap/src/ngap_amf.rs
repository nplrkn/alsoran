use super::procedures::*;
use crate::{InitiatingMessage, NgapPdu, SuccessfulOutcome};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use net::{InterfaceProvider, RequestProvider};
use slog::Logger;

pub struct NgapAmf<T>(pub T)
where
    T: RequestProvider<NgSetupRequestProcedure> + RequestProvider<RanConfigurationUpdateProcedure>;

#[async_trait]
impl<T> InterfaceProvider for NgapAmf<T>
where
    T: Send
        + Sync
        + RequestProvider<NgSetupRequestProcedure>
        + RequestProvider<RanConfigurationUpdateProcedure>,
{
    type TopPdu = NgapPdu;
    async fn route_request(&self, p: NgapPdu, logger: &Logger) -> Result<NgapPdu> {
        match match p {
            NgapPdu::InitiatingMessage(m) => m,
            _ => return Err(anyhow!("Not a request!")),
        } {
            InitiatingMessage::RanConfigurationUpdate(req) => {
                match <T as RequestProvider<RanConfigurationUpdateProcedure>>::request(
                    &self.0, req, logger,
                )
                .await
                {
                    Ok(x) => Ok(NgapPdu::SuccessfulOutcome(
                        SuccessfulOutcome::RanConfigurationUpdateAcknowledge(x),
                    )),
                    Err(_) => todo!(),
                }
            }
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
