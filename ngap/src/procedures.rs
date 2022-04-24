use crate::pdu::*;
use crate::top_pdu::*;
use xxap_transaction::*;

// Autogen this
impl IntoPdu<NgapPdu> for NgSetupRequest {
    fn into_pdu(self) -> NgapPdu {
        NgapPdu::InitiatingMessage(InitiatingMessage::NgSetupRequest(self))
    }
}

// Autogen this
pub struct NgSetupRequestProcedure {}
impl Procedure<NgapPdu> for NgSetupRequestProcedure {
    type Request = NgSetupRequest;
    type Success = NgSetupResponse;
    type Failure = NgSetupFailure;
    const CODE: u8 = 21;
}
impl Procedure2 for NgSetupRequestProcedure {
    type TopPdu = NgapPdu;
    type Request = NgSetupRequest;
    type Success = NgSetupResponse;
    type Failure = NgSetupFailure;
    const CODE: u8 = 21;
}

pub struct RanConfigurationUpdateProcedure {}
