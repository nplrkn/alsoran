mod rrc;
pub use crate::rrc::*;
use asn1_codecs::aper::{AperCodec, AperCodecData, AperCodecError};

#[derive(Clone, Debug)]
pub enum SetupRelease<T: AperCodec> {
    Release,
    Setup(T),
}

impl<T: AperCodec> AperCodec for SetupRelease<T> {
    type Output = Self;
    fn decode(_data: &mut AperCodecData) -> Result<Self, AperCodecError> {
        todo!()
    }
    fn encode(&self, _data: &mut AperCodecData) -> Result<(), AperCodecError> {
        todo!()
    }
}
