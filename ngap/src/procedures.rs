use crate::pdu::*;
use crate::top_pdu::*;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use net::Application;
use net::EventHandler;
use net::InterfaceProvider;
use net::Procedure;
use net::RequestError;
use net::RequestProvider;
use net::TnlaEvent;
use slog::Logger;

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
        let initiating_message = match p {
            NgapPdu::InitiatingMessage(m) => m,
            _ => return Err(anyhow!("Not a request!")),
        };
        match initiating_message {
            InitiatingMessage::RanConfigurationUpdate(req) => map(<T as RequestProvider<
                RanConfigurationUpdateProcedure,
            >>::request(
                &self.0, req, logger
            )
            .await),
            InitiatingMessage::NgSetupRequest(req) => {
                map(
                    <T as RequestProvider<NgSetupRequestProcedure>>::request(&self.0, req, logger)
                        .await,
                )
            }
            _ => todo!(),
        }
    }
}

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
                map(
                    <T as RequestProvider<NgSetupRequestProcedure>>::request(&self.0, req, logger)
                        .await,
                )
            }
            _ => todo!(),
        }
    }
}
