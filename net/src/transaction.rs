use anyhow::Result;
use asn1_codecs::aper::{self, AperCodecData};
use async_channel::RecvError;
use async_trait::async_trait;
use slog::{debug, warn, Logger};
use std::fmt::Debug;

#[async_trait]
pub trait Procedure {
    const CODE: u8;
    type TopPdu: AperSerde + Send + Sync + 'static;
    type Request: Send + Sync + 'static + Debug;
    type Success;
    type Failure;
    fn encode_request(r: Self::Request) -> Result<Vec<u8>, AperCodecError>;
    fn decode_response(bytes: &[u8]) -> Result<Self::Success, RequestError<Self::Failure>>;
    async fn call_provider<T: RequestProvider<Self>>(
        provider: &T,
        req: Self::Request,
        logger: &Logger,
    ) -> Option<Self::TopPdu>;
}

#[async_trait]
pub trait Indication {
    const CODE: u8;
    type TopPdu: AperSerde + Send + Sync + 'static;
    type Request: Send + Sync + 'static + Debug;
    fn encode_request(r: Self::Request) -> Result<Vec<u8>, AperCodecError>;
    async fn call_provider<T: IndicationHandler<Self>>(
        provider: &T,
        req: Self::Request,
        logger: &Logger,
    );
}

pub trait AperSerde: Sized {
    fn into_bytes(self) -> Result<Vec<u8>, AperCodecError>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, AperCodecError>;
}
pub use aper::AperCodecError;

impl<T: aper::AperCodec<Output = T>> AperSerde for T {
    fn into_bytes(self) -> Result<Vec<u8>, AperCodecError> {
        let mut d = AperCodecData::new();
        self.encode(&mut d)?;
        Ok(d.into_bytes())
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, AperCodecError> {
        let mut d = AperCodecData::from_slice(bytes);
        Self::decode(&mut d)
    }
}

#[derive(Debug)]
pub enum RequestError<U> {
    UnsuccessfulOutcome(U),
    Other(String),
}

impl<T> From<AperCodecError> for RequestError<T> {
    fn from(e: AperCodecError) -> Self {
        RequestError::Other(format!("Codec error: {:?}", e))
    }
}

impl<T> From<RecvError> for RequestError<T> {
    fn from(e: RecvError) -> Self {
        RequestError::Other(format!("Channel recv error: {:?}", e))
    }
}

impl<T> From<anyhow::Error> for RequestError<T> {
    fn from(e: anyhow::Error) -> Self {
        RequestError::Other(format!("Transport error: {:?}", e))
    }
}

/// Trait representing the ability to handle a single procedure.
#[async_trait]
pub trait RequestProvider<P: Procedure + ?Sized>: Send + Sync {
    async fn request(
        &self,
        r: P::Request,
        logger: &Logger,
    ) -> Result<P::Success, RequestError<P::Failure>> {
        debug!(logger, "Received unimplemented request {:?}", r);
        Err(RequestError::Other("Not implemented".to_string()))
    }
}

/// Trait representing the ability to handle an indication.
#[async_trait]
pub trait IndicationHandler<I: Indication + ?Sized>: Send + Sync {
    async fn handle(&self, i: I::Request, logger: &Logger) {
        warn!(logger, "Received unimplemented indication {:?}", i);
    }
}

/// Trait representing the ability to handle multiple procedures that use the same top level PDU.
#[async_trait]
pub trait InterfaceProvider: Send + Sync {
    type TopPdu: AperSerde;
    async fn route_request(&self, p: Self::TopPdu, logger: &Logger) -> Option<Self::TopPdu>;
}

/// Trait representing the ability to handle and respond to a request in wire format.
#[async_trait]
pub trait RequestMessageHandler: Send + Sync {
    async fn handle_request(&self, message: &[u8], logger: &Logger) -> Option<Vec<u8>>;
}

// An interface provider is a request message handler.
#[async_trait]
impl<T: AperSerde + Send + Sync, I: InterfaceProvider<TopPdu = T>> RequestMessageHandler for I {
    async fn handle_request(&self, message: &[u8], logger: &Logger) -> Option<Vec<u8>> {
        let pdu = match T::from_bytes(message) {
            Ok(p) => p,
            Err(e) => {
                warn!(logger, "ASN.1 decode failed - {:?}", e);
                return None;
            }
        };
        match self
            .route_request(pdu, logger)
            .await
            .map(|x| x.into_bytes())
        {
            None => None,
            Some(Ok(bytes)) => Some(bytes),
            Some(Err(e)) => {
                warn!(logger, "ASN.1 encode failed - {:?}", e);
                None
            }
        }
    }
}
