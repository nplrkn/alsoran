use crate::pdu::*;
use crate::top_pdu::*;
use asn1_codecs::aper::{AperCodec, AperCodecData, AperCodecError};
use xxap_transaction::*;

// Autogen this
impl From<F1SetupRequest> for F1apPdu {
    fn from(x: F1SetupRequest) -> Self {
        F1apPdu::InitiatingMessage(InitiatingMessage::F1SetupRequest(x))
    }
}

// Autogen this
pub struct F1SetupProcedure {}
impl Procedure for F1SetupProcedure {
    type TopPdu = F1apPdu;
    type Request = F1SetupRequest;
    type Success = F1SetupResponse;
    type Failure = F1SetupFailure;
    const CODE: u8 = 1;
}

pub struct RanConfigurationUpdateProcedure {}
