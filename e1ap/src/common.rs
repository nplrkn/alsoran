// Autogenerated from E1AP-CommonDataTypes.asn
use asn1_codecs::aper::{self, AperCodec, AperCodecData, AperCodecError};
use bitvec::prelude::*;
#[allow(dead_code)]
pub type BitString = BitVec<u8, Msb0>;
#[allow(unused_imports)]
use num_enum::TryFromPrimitive;

// Criticality
#[derive(Clone, Debug, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum Criticality {
    Reject,
    Ignore,
    Notify,
}

impl Criticality {
    fn decode_inner(data: &mut AperCodecData) -> Result<Self, AperCodecError> {
        let (idx, extended) = aper::decode::decode_enumerated(data, Some(0), Some(2), false)?;
        if extended {
            return Err(aper::AperCodecError::new("Extended enum not implemented"));
        }
        Self::try_from(idx as u8).map_err(|_| AperCodecError::new("Unknown enum variant"))
    }
    fn encode_inner(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        aper::encode::encode_enumerated(data, Some(0), Some(2), false, *self as i128, false)
    }
}

impl AperCodec for Criticality {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self, AperCodecError> {
        Criticality::decode_inner(data).map_err(|e: AperCodecError| e.push_context("Criticality"))
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        self.encode_inner(data)
            .map_err(|e: AperCodecError| e.push_context("Criticality"))
    }
}
// Presence
#[derive(Clone, Debug, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum Presence {
    Optional,
    Conditional,
    Mandatory,
}

impl Presence {
    fn decode_inner(data: &mut AperCodecData) -> Result<Self, AperCodecError> {
        let (idx, extended) = aper::decode::decode_enumerated(data, Some(0), Some(2), false)?;
        if extended {
            return Err(aper::AperCodecError::new("Extended enum not implemented"));
        }
        Self::try_from(idx as u8).map_err(|_| AperCodecError::new("Unknown enum variant"))
    }
    fn encode_inner(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        aper::encode::encode_enumerated(data, Some(0), Some(2), false, *self as i128, false)
    }
}

impl AperCodec for Presence {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self, AperCodecError> {
        Presence::decode_inner(data).map_err(|e: AperCodecError| e.push_context("Presence"))
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        self.encode_inner(data)
            .map_err(|e: AperCodecError| e.push_context("Presence"))
    }
}
// PrivateIeId
#[derive(Clone, Debug)]
pub enum PrivateIeId {
    Local(u16),
    Global(Vec<u8>),
}

impl PrivateIeId {
    fn decode_inner(data: &mut AperCodecData) -> Result<Self, AperCodecError> {
        let (idx, extended) = aper::decode::decode_choice_idx(data, 0, 1, false)?;
        if extended {
            return Err(aper::AperCodecError::new(
                "CHOICE additions not implemented",
            ));
        }
        match idx {
            0 => Ok(Self::Local(
                aper::decode::decode_integer(data, Some(0), Some(65535), false)?.0 as u16,
            )),
            1 => Ok(Self::Global(aper::decode::decode_octetstring(
                data, None, None, false,
            )?)),
            _ => Err(AperCodecError::new("Unknown choice idx")),
        }
    }
    fn encode_inner(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        match self {
            Self::Local(x) => {
                aper::encode::encode_choice_idx(data, 0, 1, false, 0, false)?;
                aper::encode::encode_integer(data, Some(0), Some(65535), false, *x as i128, false)
            }
            Self::Global(x) => {
                aper::encode::encode_choice_idx(data, 0, 1, false, 1, false)?;
                aper::encode::encode_octetstring(data, None, None, false, &x, false)
            }
        }
    }
}

impl AperCodec for PrivateIeId {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self, AperCodecError> {
        PrivateIeId::decode_inner(data).map_err(|e: AperCodecError| e.push_context("PrivateIeId"))
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        self.encode_inner(data)
            .map_err(|e: AperCodecError| e.push_context("PrivateIeId"))
    }
}
// ProcedureCode
#[derive(Clone, Debug)]
pub struct ProcedureCode(pub u8);

impl ProcedureCode {
    fn decode_inner(data: &mut AperCodecData) -> Result<Self, AperCodecError> {
        Ok(Self(
            aper::decode::decode_integer(data, Some(0), Some(255), false)?.0 as u8,
        ))
    }
    fn encode_inner(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        aper::encode::encode_integer(data, Some(0), Some(255), false, self.0 as i128, false)
    }
}

impl AperCodec for ProcedureCode {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self, AperCodecError> {
        ProcedureCode::decode_inner(data)
            .map_err(|e: AperCodecError| e.push_context("ProcedureCode"))
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        self.encode_inner(data)
            .map_err(|e: AperCodecError| e.push_context("ProcedureCode"))
    }
}
// ProtocolExtensionId
#[derive(Clone, Debug)]
pub struct ProtocolExtensionId(pub u16);

impl ProtocolExtensionId {
    fn decode_inner(data: &mut AperCodecData) -> Result<Self, AperCodecError> {
        Ok(Self(
            aper::decode::decode_integer(data, Some(0), Some(65535), false)?.0 as u16,
        ))
    }
    fn encode_inner(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        aper::encode::encode_integer(data, Some(0), Some(65535), false, self.0 as i128, false)
    }
}

impl AperCodec for ProtocolExtensionId {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self, AperCodecError> {
        ProtocolExtensionId::decode_inner(data)
            .map_err(|e: AperCodecError| e.push_context("ProtocolExtensionId"))
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        self.encode_inner(data)
            .map_err(|e: AperCodecError| e.push_context("ProtocolExtensionId"))
    }
}
// ProtocolIeId
#[derive(Clone, Debug)]
pub struct ProtocolIeId(pub u16);

impl ProtocolIeId {
    fn decode_inner(data: &mut AperCodecData) -> Result<Self, AperCodecError> {
        Ok(Self(
            aper::decode::decode_integer(data, Some(0), Some(65535), false)?.0 as u16,
        ))
    }
    fn encode_inner(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        aper::encode::encode_integer(data, Some(0), Some(65535), false, self.0 as i128, false)
    }
}

impl AperCodec for ProtocolIeId {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self, AperCodecError> {
        ProtocolIeId::decode_inner(data).map_err(|e: AperCodecError| e.push_context("ProtocolIeId"))
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        self.encode_inner(data)
            .map_err(|e: AperCodecError| e.push_context("ProtocolIeId"))
    }
}
// TriggeringMessage
#[derive(Clone, Debug, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum TriggeringMessage {
    InitiatingMessage,
    SuccessfulOutcome,
    UnsuccessfulOutcome,
}

impl TriggeringMessage {
    fn decode_inner(data: &mut AperCodecData) -> Result<Self, AperCodecError> {
        let (idx, extended) = aper::decode::decode_enumerated(data, Some(0), Some(2), false)?;
        if extended {
            return Err(aper::AperCodecError::new("Extended enum not implemented"));
        }
        Self::try_from(idx as u8).map_err(|_| AperCodecError::new("Unknown enum variant"))
    }
    fn encode_inner(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        aper::encode::encode_enumerated(data, Some(0), Some(2), false, *self as i128, false)
    }
}

impl AperCodec for TriggeringMessage {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self, AperCodecError> {
        TriggeringMessage::decode_inner(data)
            .map_err(|e: AperCodecError| e.push_context("TriggeringMessage"))
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        self.encode_inner(data)
            .map_err(|e: AperCodecError| e.push_context("TriggeringMessage"))
    }
}