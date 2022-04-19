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

use asn1_codecs::aper::{AperCodec, AperCodecData};

#[derive(Clone)]
pub struct Asn1PerCodec<P>(pub PhantomData<P>)
where
    P: Send + Sync + Clone + AperCodec;
impl<P> Asn1PerCodec<P>
where
    P: Send + Sync + Clone + AperCodec,
{
    pub fn new() -> Asn1PerCodec<P> {
        Asn1PerCodec(PhantomData)
    }
}

impl<P> Default for Asn1PerCodec<P>
where
    P: Send + Sync + Clone + AperCodec,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<P> Codec for Asn1PerCodec<P>
where
    P: Send + Sync + Clone + AperCodec<Output = P>,
{
    type Pdu = P;
    fn to_wire(&self, pdu: Self::Pdu) -> Result<Vec<u8>> {
        let mut data = AperCodecData::new();
        pdu.encode(&mut data)?;
        Ok(data.into_bytes())
    }
    fn from_wire(&self, message: Vec<u8>) -> Result<Self::Pdu> {
        let mut data = AperCodecData::from_slice(&message);
        let pdu = Self::Pdu::decode(&mut data)?;
        Ok(pdu)
    }
}
