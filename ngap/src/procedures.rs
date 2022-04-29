use crate::pdu::*;
use crate::top_pdu::*;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use slog::Logger;
use xxap_transaction::*;

// Autogen or derive this
impl From<NgSetupRequest> for NgapPdu {
    fn from(x: NgSetupRequest) -> Self {
        NgapPdu::InitiatingMessage(InitiatingMessage::NgSetupRequest(x))
    }
}
impl From<RanConfigurationUpdate> for NgapPdu {
    fn from(x: RanConfigurationUpdate) -> Self {
        NgapPdu::InitiatingMessage(InitiatingMessage::RanConfigurationUpdate(x))
    }
}
impl From<NgSetupResponse> for SuccessfulOutcome {
    fn from(x: NgSetupResponse) -> Self {
        SuccessfulOutcome::NgSetupResponse(x)
    }
}
impl From<NgSetupFailure> for UnsuccessfulOutcome {
    fn from(x: NgSetupFailure) -> Self {
        UnsuccessfulOutcome::NgSetupFailure(x)
    }
}

impl From<RanConfigurationUpdateAcknowledge> for SuccessfulOutcome {
    fn from(x: RanConfigurationUpdateAcknowledge) -> Self {
        SuccessfulOutcome::RanConfigurationUpdateAcknowledge(x)
    }
}
impl From<RanConfigurationUpdateFailure> for UnsuccessfulOutcome {
    fn from(x: RanConfigurationUpdateFailure) -> Self {
        UnsuccessfulOutcome::RanConfigurationUpdateFailure(x)
    }
}

// Autogen this
pub struct NgSetupRequestProcedure {}
impl Procedure for NgSetupRequestProcedure {
    type TopPdu = NgapPdu;
    type Request = NgSetupRequest;
    type Success = NgSetupResponse;
    type Failure = NgSetupFailure;
    const CODE: u8 = 21;
}

pub struct RanConfigurationUpdateProcedure {}
impl Procedure for RanConfigurationUpdateProcedure {
    type TopPdu = NgapPdu;
    type Request = RanConfigurationUpdate;
    type Success = RanConfigurationUpdateAcknowledge;
    type Failure = RanConfigurationUpdateFailure;
    const CODE: u8 = 22;
}

fn map<T, E>(r: Result<T, RequestError<E>>) -> Result<NgapPdu>
where
    UnsuccessfulOutcome: From<E>,
    SuccessfulOutcome: From<T>,
{
    r.map(|x| NgapPdu::SuccessfulOutcome(x.into()))
        .or_else(|e| match e {
            RequestError::UnsuccessfulOutcome(x) => Ok(NgapPdu::UnsuccessfulOutcome(x.into())),
            RequestError::Other(s) => Err(anyhow!(format!("{}", s))),
        })
}

#[async_trait]
pub trait NgapAmfProvider:
    RequestProvider<NgSetupRequestProcedure> + RequestProvider<RanConfigurationUpdateProcedure>
{
    async fn route_request(&self, p: InitiatingMessage, logger: &Logger) -> Result<NgapPdu> {
        match p {
            InitiatingMessage::RanConfigurationUpdate(req) => map(<Self as RequestProvider<
                RanConfigurationUpdateProcedure,
            >>::request(
                self, req, logger
            )
            .await),
            InitiatingMessage::NgSetupRequest(req) => {
                map(
                    <Self as RequestProvider<NgSetupRequestProcedure>>::request(self, req, logger)
                        .await,
                )
            }
            _ => todo!(),
        }
    }
}

// So the Stack should provide this.  And the Gnb biz logic should take one of these.

#[async_trait]
pub trait NgapGnbProvider: RequestProvider<RanConfigurationUpdateProcedure> {
    async fn route_request(&self, p: InitiatingMessage, logger: &Logger) -> Result<NgapPdu> {
        match p {
            InitiatingMessage::RanConfigurationUpdate(req) => map(<Self as RequestProvider<
                RanConfigurationUpdateProcedure,
            >>::request(
                self, req, logger
            )
            .await),
            _ => todo!(),
        }
    }
}
