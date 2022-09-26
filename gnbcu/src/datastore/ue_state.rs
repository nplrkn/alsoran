//! ue_state - serializable model of GNB-CU's per UE state

use e1ap::GnbCuUpUeE1apId;
use f1ap::GnbDuUeF1apId;
use ngap::AmfUeNgapId;
use speedy::{Readable, Writable};

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
    pub fn new(key: u32, gnb_du_ue_f1ap_id: GnbDuUeF1apId) -> Self {
        UeState {
            key,
            gnb_du_ue_f1ap_id,
            gnb_cu_up_ue_e1ap_id: None,
            amf_ue_ngap_id: None,
        }
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
            amf_ue_ngap_id: x.amf_ue_ngap_id.map(|x| AmfUeNgapId(x)),
            gnb_cu_up_ue_e1ap_id: x.gnb_cu_up_ue_e1ap_id.map(|x| GnbCuUpUeE1apId(x)),
        }
    }
}
