use asn1_codecs::aper::AperCodec;
use async_trait::async_trait;
use slog::Logger;
use std::fmt::Debug;

// pub trait Procedure<T: AperCodec> {
//     const CODE: u8;
//     type Request: AperCodec + IntoPdu<T> + Send + Sync + 'static;
//     type Success: AperCodec<Output = Self::Success>;
//     type Failure: AperCodec<Output = Self::Failure>;
// }

pub trait Procedure {
    const CODE: u8;
    type TopPdu: AperCodec;
    type Request: AperCodec + IntoPdu<Self::TopPdu> + Send + Sync + 'static;
    type Success: AperCodec<Output = Self::Success>;
    type Failure: AperCodec<Output = Self::Failure>;
}

pub trait IntoPdu<P> {
    fn into_pdu(self) -> P;
}

pub enum RequestError<U> {
    UnsuccessfulResponse(U),
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
    ) -> Result<P::Success, RequestError<P::Failure>>;
}
