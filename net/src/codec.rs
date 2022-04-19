use std::marker::PhantomData;

pub trait Codec: Send + Sync + Clone {
    type Pdu;
    fn to_wire(&self, pdu: Self::Pdu) -> Result<Vec<u8>>;
    fn from_wire(&self, message: Vec<u8>) -> Result<Self::Pdu>;
}

use anyhow::Result;
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
        let pdu = Self::Pdu::decode(&mut data).unwrap();
        Ok(pdu)
    }
}
