use super::top_pdu::*;
use crate::{InitiatingMessage, NgapPdu};
use async_trait::async_trait;
use net::{InterfaceProvider, Procedure, RequestProvider};
use slog::Logger;

pub struct NgapAmf<T>(pub T)
where
    T: RequestProvider<NgSetupProcedure> + RequestProvider<RanConfigurationUpdateProcedure>;

#[async_trait]
impl<T> InterfaceProvider for NgapAmf<T>
where
    T: Send
        + Sync
        + RequestProvider<NgSetupProcedure>
        + RequestProvider<RanConfigurationUpdateProcedure>,
{
    type TopPdu = NgapPdu;
    async fn route_request(&self, p: NgapPdu, logger: &Logger) -> Option<NgapPdu> {
        match match p {
            NgapPdu::InitiatingMessage(m) => m,
            _ => return None,
        } {
            InitiatingMessage::RanConfigurationUpdate(req) => {
                RanConfigurationUpdateProcedure::call_provider(&self.0, req, logger).await
            }
            InitiatingMessage::NgSetupRequest(req) => {
                NgSetupProcedure::call_provider(&self.0, req, logger).await
            }
            _ => todo!(),
        }
    }
}
