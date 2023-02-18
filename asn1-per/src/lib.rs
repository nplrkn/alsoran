mod transaction;
pub use transaction::*;

pub use asn1_codecs::{PerCodecData, PerCodecError};
use bitvec::prelude::*;
pub type BitString = BitVec<u8, Msb0>;
pub use num_enum::TryFromPrimitive;

pub trait PerCodec: Sized {
    fn decode(data: &mut PerCodecData) -> Result<Self, crate::PerCodecError>;
    fn encode(&self, _data: &mut PerCodecData) -> Result<(), crate::PerCodecError>;
}

pub mod aper {
    pub use asn1_codecs::aper::{decode, encode};
    pub fn new_codec_data() -> crate::PerCodecData {
        crate::PerCodecData::new_aper()
    }
}

pub mod uper {
    pub use asn1_codecs::uper::{decode, encode};
    pub fn new_codec_data() -> crate::PerCodecData {
        crate::PerCodecData::new_uper()
    }
}
