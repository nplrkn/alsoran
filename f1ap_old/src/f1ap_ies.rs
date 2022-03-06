use bitvec::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionId(pub u8);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GnbDuId(pub u64);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RrcVersion {
    pub latest_rrc_version: BitVec<Msb0, u8>,
    pub latest_rrc_version_enhanced: Option<[u8; 3]>,
}
impl RrcVersion {
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        RrcVersion {
            latest_rrc_version: bitvec![Msb0, u8; 0; 3],
            latest_rrc_version_enhanced: Some([x, y, z]),
        }
    }
}
