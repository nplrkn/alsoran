use anyhow::Result;
use asn1_codecs::aper::{self, AperCodecData};
use async_channel::RecvError;
use async_trait::async_trait;
use slog::{trace, Logger};
use std::fmt::Debug;

pub trait Procedure {
    const CODE: u8;
    type TopPdu: AperCodec + Send + Sync + 'static;
    type Request: Send + Sync + 'static + Debug;
    type Success;
    type Failure;
    fn encode_request(r: Self::Request) -> Result<Vec<u8>, AperCodecError>;
    fn decode_response(bytes: &[u8]) -> Result<Self::Success, RequestError<Self::Failure>>;
}

// This replaces AperCodec from the asn1_codecs crate with a more ergonomic version.
pub trait AperCodec: Sized {
    fn into_bytes(self) -> Result<Vec<u8>, AperCodecError>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, AperCodecError>;
}
pub use aper::AperCodecError;

impl<T: aper::AperCodec<Output = T>> AperCodec for T {
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

// impl<U, D: Debug> From<D> for RequestError<U> {
//     fn from(e: D) -> Self {
//         RequestError::Other(format!("{:?}", e))
//     }
// }

/// Trait representing the ability to handle a single procedure.
#[async_trait]
pub trait RequestProvider<P: Procedure> {
    async fn request(
        &self,
        r: P::Request,
        logger: &Logger,
    ) -> Result<P::Success, RequestError<P::Failure>> {
        trace!(logger, "Received unimplemented request {:?}", r);
        Err(RequestError::Other("Not implemented".to_string()))
    }
}

/// Trait representing the ability to handle multiple procedures that use the same top level PDU.
#[async_trait]
pub trait InterfaceProvider: Send + Sync {
    type TopPdu: AperCodec;
    async fn route_request(&self, p: Self::TopPdu, logger: &Logger) -> Result<Self::TopPdu>;
}

/// Trait representing the ability to handle and respond to a request in wire format.
#[async_trait]
pub trait RequestMessageHandler: Send + Sync {
    async fn handle_request(&self, message: &[u8], logger: &Logger) -> Result<Vec<u8>>;
}

// An interface provider is a request message handler.
#[async_trait]
impl<T: AperCodec + Send + Sync, I: InterfaceProvider<TopPdu = T>> RequestMessageHandler for I {
    async fn handle_request(&self, message: &[u8], logger: &Logger) -> Result<Vec<u8>> {
        let pdu = T::from_bytes(message)?;
        Ok(self.route_request(pdu, logger).await?.into_bytes()?)
    }
}
