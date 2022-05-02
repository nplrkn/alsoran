use crate::pdu::*;
use crate::top_pdu::*;
use anyhow::Result;
use net::AperCodec;
use net::AperCodecError;
use net::Procedure;
use net::RequestError;

// Autogen or derive this
// impl From<NgSetupRequest> for NgapPdu {
//     fn from(x: NgSetupRequest) -> Self {
//         NgapPdu::InitiatingMessage(InitiatingMessage::NgSetupRequest(x))
//     }
// }
// impl From<RanConfigurationUpdate> for NgapPdu {
//     fn from(x: RanConfigurationUpdate) -> Self {
//         NgapPdu::InitiatingMessage(InitiatingMessage::RanConfigurationUpdate(x))
//     }
// }
// impl From<NgSetupResponse> for SuccessfulOutcome {
//     fn from(x: NgSetupResponse) -> Self {
//         SuccessfulOutcome::NgSetupResponse(x)
//     }
// }
// impl From<NgSetupFailure> for UnsuccessfulOutcome {
//     fn from(x: NgSetupFailure) -> Self {
//         UnsuccessfulOutcome::NgSetupFailure(x)
//     }
// }

// impl From<SuccessfulOutcome> for NgapPdu {
//     fn from(x: SuccessfulOutcome) -> Self {
//         NgapPdu::SuccessfulOutcome(x)
//     }
// }

// impl TryFrom<NgapPdu> for SuccessfulOutcome {
//     type Error = ();
//     fn try_from(x: NgapPdu) -> Result<Self, ()> {
//         match x {
//             NgapPdu::SuccessfulOutcome(x) => Ok(x),
//             _ => Err(()),
//         }
//     }
// }

// impl From<UnsuccessfulOutcome> for NgapPdu {
//     fn from(x: UnsuccessfulOutcome) -> Self {
//         NgapPdu::UnsuccessfulOutcome(x)
//     }
// }

// impl From<RanConfigurationUpdateAcknowledge> for SuccessfulOutcome {
//     fn from(x: RanConfigurationUpdateAcknowledge) -> Self {
//         SuccessfulOutcome::RanConfigurationUpdateAcknowledge(x)
//     }
// }
// impl From<RanConfigurationUpdateFailure> for UnsuccessfulOutcome {
//     fn from(x: RanConfigurationUpdateFailure) -> Self {
//         UnsuccessfulOutcome::RanConfigurationUpdateFailure(x)
//     }
// }

// Autogen this
pub struct NgSetupRequestProcedure {}
impl Procedure for NgSetupRequestProcedure {
    type TopPdu = NgapPdu;
    type Request = NgSetupRequest;
    type Success = NgSetupResponse;
    type Failure = NgSetupFailure;
    const CODE: u8 = 21;
    fn encode_request(r: Self::Request) -> Result<Vec<u8>, AperCodecError> {
        NgapPdu::InitiatingMessage(InitiatingMessage::NgSetupRequest(r)).into_bytes()
    }

    fn decode_response(bytes: &[u8]) -> Result<Self::Success, RequestError<Self::Failure>> {
        let response_pdu = Self::TopPdu::from_bytes(bytes)?;
        match response_pdu {
            NgapPdu::SuccessfulOutcome(SuccessfulOutcome::NgSetupResponse(x)) => Ok(x),
            NgapPdu::UnsuccessfulOutcome(UnsuccessfulOutcome::NgSetupFailure(x)) => {
                Err(RequestError::UnsuccessfulOutcome(x))
            }
            _ => Err(RequestError::Other("Unexpected pdu contents".to_string())),
        }
    }
}

// impl TryFrom<Result<NgSetupResponse, RequestError<NgSetupFailure>>> for NgapPdu {
//     type Error = anyhow::Error;
//     fn try_from(x: Result<NgSetupResponse, RequestError<NgSetupFailure>>) -> Result<NgapPdu> {
//         todo!()
//     }
// }

pub struct RanConfigurationUpdateProcedure {}
impl Procedure for RanConfigurationUpdateProcedure {
    type TopPdu = NgapPdu;
    type Request = RanConfigurationUpdate;
    type Success = RanConfigurationUpdateAcknowledge;
    type Failure = RanConfigurationUpdateFailure;
    const CODE: u8 = 22;
    fn encode_request(r: Self::Request) -> Result<Vec<u8>, AperCodecError> {
        NgapPdu::InitiatingMessage(InitiatingMessage::RanConfigurationUpdate(r)).into_bytes()
    }

    fn decode_response(bytes: &[u8]) -> Result<Self::Success, RequestError<Self::Failure>> {
        let response_pdu = Self::TopPdu::from_bytes(bytes)?;
        match response_pdu {
            NgapPdu::SuccessfulOutcome(SuccessfulOutcome::RanConfigurationUpdateAcknowledge(x)) => {
                Ok(x)
            }
            NgapPdu::UnsuccessfulOutcome(UnsuccessfulOutcome::RanConfigurationUpdateFailure(x)) => {
                Err(RequestError::UnsuccessfulOutcome(x))
            }
            _ => Err(RequestError::Other("Unexpected pdu contents".to_string())),
        }
    }
}
