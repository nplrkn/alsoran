use asn1_codecs::aper;
use async_trait::async_trait;
use slog::{trace, Logger};
use std::fmt::Debug;

pub trait Procedure {
    const CODE: u8;
    type TopPdu: AperCodec;
    type Request: AperCodec + Into<Self::TopPdu> + Send + Sync + 'static + Debug;
    type Success: AperCodec;
    type Failure: AperCodec;
}

// This replaces AperCodec from the asn1_codecs crate with a more ergonomic version.
pub trait AperCodec: Sized {
    fn into_bytes(self) -> Result<Vec<u8>, AperCodecError>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, AperCodecError>;
}
pub use aper::AperCodecError;

impl<T: aper::AperCodec> AperCodec for T {
    fn into_bytes(self) -> Result<Vec<u8>, AperCodecError> {
        todo!()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, AperCodecError> {
        todo!()
    }
}

// pub trait IntoPduBytes<P> {
//     fn into_pdu_bytes(self) -> Result<Vec<u8>, AperCodecError>;
// }

pub enum RequestError<U> {
    UnsuccessfulOutcome(U),
    Other(String),
}

impl<U, D: Debug> From<D> for RequestError<U> {
    fn from(e: D) -> Self {
        RequestError::Other(format!("{:?}", e))
    }
}

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
