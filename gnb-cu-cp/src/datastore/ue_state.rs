//! ue_state - serializable model of GNB-CU's per UE state

use super::SerDes;
use super::StateStore;
use anyhow::Result;
use e1ap::GnbCuUpUeE1apId;
use f1ap::GnbDuUeF1apId;
use ngap::AmfUeNgapId;
use rand::Rng;
use speedy::{Readable, Writable};

pub trait UeStateStore: StateStore<UeState> {}

#[derive(Clone, Debug)]
pub struct UeState {
    pub key: u32,
    pub gnb_du_ue_f1ap_id: GnbDuUeF1apId,
    pub gnb_cu_up_ue_e1ap_id: Option<GnbCuUpUeE1apId>,
    pub amf_ue_ngap_id: Option<AmfUeNgapId>,
}

#[derive(Readable, Writable)]
pub struct UeStateSerializable {
    pub key: u32,
    pub gnb_du_ue_f1ap_id: u32,
    pub amf_ue_ngap_id: Option<u64>,
    pub gnb_cu_up_ue_e1ap_id: Option<u32>,
}

impl UeState {
    pub fn new(gnb_du_ue_f1ap_id: GnbDuUeF1apId) -> Self {
        UeState {
            key: rand::thread_rng().gen::<u32>(),
            gnb_du_ue_f1ap_id,
            gnb_cu_up_ue_e1ap_id: None,
            amf_ue_ngap_id: None,
        }
    }
}

impl SerDes for UeState {
    fn into_bytes(self) -> Result<Vec<u8>> {
        let s: UeStateSerializable = self.into();
        Ok(s.write_to_vec()?)
    }
    fn from_bytes(v: &[u8]) -> Result<Self> {
        let s = UeStateSerializable::read_from_buffer(v)?;
        Ok(s.into())
    }
}

impl From<UeState> for UeStateSerializable {
    fn from(x: UeState) -> Self {
        UeStateSerializable {
            key: x.key,
            gnb_du_ue_f1ap_id: x.gnb_du_ue_f1ap_id.0,
            amf_ue_ngap_id: x.amf_ue_ngap_id.map(|x| x.0),
            gnb_cu_up_ue_e1ap_id: x.gnb_cu_up_ue_e1ap_id.map(|x| x.0),
        }
    }
}

impl From<UeStateSerializable> for UeState {
    fn from(x: UeStateSerializable) -> Self {
        UeState {
            key: x.key,
            gnb_du_ue_f1ap_id: GnbDuUeF1apId(x.gnb_du_ue_f1ap_id),
            amf_ue_ngap_id: x.amf_ue_ngap_id.map(AmfUeNgapId),
            gnb_cu_up_ue_e1ap_id: x.gnb_cu_up_ue_e1ap_id.map(GnbCuUpUeE1apId),
        }
    }
}
