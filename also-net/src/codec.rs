use std::marker::PhantomData;

pub trait Codec: Send + Sync + Clone {
    type Pdu;
    fn to_wire(&self, pdu: Self::Pdu) -> Result<Vec<u8>>;
    fn from_wire(&self, message: Vec<u8>) -> Result<Self::Pdu>;
}

use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Clone)]
pub struct JsonCodec<P>(pub PhantomData<P>)
where
    P: Send + Sync + Clone;
impl<P> JsonCodec<P>
where
    P: Send + Sync + Clone,
{
    pub fn new() -> JsonCodec<P> {
        JsonCodec(PhantomData)
    }
}
impl<P> Default for JsonCodec<P>
where
    P: Send + Sync + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<P> Codec for JsonCodec<P>
where
    P: Serialize + DeserializeOwned + Send + Sync + Clone,
{
    type Pdu = P;
    fn to_wire(&self, pdu: Self::Pdu) -> Result<Vec<u8>> {
        let string = serde_json::to_string(&pdu)?;
        Ok(string.into())
    }
    fn from_wire(&self, message: Vec<u8>) -> Result<Self::Pdu> {
        Ok(serde_json::from_slice(&message)?)
    }
}

#[derive(Clone)]
pub struct Asn1PerCodec<P>(pub PhantomData<P>)
where
    P: Send + Sync + Clone;
impl<P> Asn1PerCodec<P>
where
    P: Send + Sync + Clone,
{
    pub fn new() -> Asn1PerCodec<P> {
        Asn1PerCodec(PhantomData)
    }
}

impl<P> Default for Asn1PerCodec<P>
where
    P: Send + Sync + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<P> Codec for Asn1PerCodec<P>
where
    P: Send + Sync + Clone,
{
    type Pdu = P;
    fn to_wire(&self, _pdu: Self::Pdu) -> Result<Vec<u8>> {
        Ok(hex::decode("00150035000004001b00080002f83910000102005240090300667265653567630066001000000000010002f839000010080102030015400140")?)
    }
    fn from_wire(&self, _message: Vec<u8>) -> Result<Self::Pdu> {
        unimplemented!()
    }
}
