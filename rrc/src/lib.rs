mod rrc;
pub use crate::rrc::*;
use asn1_codecs::{aper::AperCodec, PerCodecData, PerCodecError};
mod procedure;
pub use procedure::*;

#[derive(Clone, Debug)]
pub enum SetupRelease<T: AperCodec> {
    Release,
    Setup(T),
}

impl<T: AperCodec> AperCodec for SetupRelease<T> {
    type Output = Self;
    fn aper_decode(_data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        todo!()
    }
    fn aper_encode(&self, _data: &mut PerCodecData) -> Result<(), PerCodecError> {
        todo!()
    }
}
