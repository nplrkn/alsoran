//! ngap_amf - Collects together the procedures that are served by an AMF on the NG reference point.

use super::top_pdu::*;
use crate::{InitiatingMessage, NgapPdu};
use asn1_per::*;
use async_trait::async_trait;
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
    async fn route_request(&self, p: NgapPdu, logger: &Logger) -> Option<ResponseAction<NgapPdu>> {
        match p {
            NgapPdu::InitiatingMessage(InitiatingMessage::RanConfigurationUpdate(req)) => {
                RanConfigurationUpdateProcedure::call_provider(&self.0, req, logger).await
            }
            NgapPdu::InitiatingMessage(InitiatingMessage::NgSetupRequest(req)) => {
                NgSetupProcedure::call_provider(&self.0, req, logger).await
            }
            _ => return None,
        }
    }
}
