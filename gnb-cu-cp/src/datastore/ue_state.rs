//! ue_state - serializable model of GNB-CU's per UE state

use super::SerDes;
use super::StateStore;
use anyhow::Result;
use asn1_per::SerDes as Asn1Serdes;
use e1ap::GnbCuUpUeE1apId;
use f1ap::{GnbDuUeF1apId, NrCgi};
use ngap::AmfUeNgapId;
use rand::Rng;
use speedy::{Readable, Writable};

pub trait UeStateStore: StateStore<UeState> {}

#[derive(Clone, Debug)]
pub struct UeState {
    pub key: u32,
    pub gnb_du_ue_f1ap_id: GnbDuUeF1apId,
    pub nr_cgi: NrCgi,
    pub gnb_cu_up_ue_e1ap_id: Option<GnbCuUpUeE1apId>,
    pub amf_ue_ngap_id: Option<AmfUeNgapId>,
}

#[derive(Readable, Writable)]
pub struct UeStateSerializable {
    pub key: u32,
    pub gnb_du_ue_f1ap_id: u32,
    pub nr_cgi: Vec<u8>,
    pub amf_ue_ngap_id: Option<u64>,
    pub gnb_cu_up_ue_e1ap_id: Option<u32>,
}

impl UeState {
    pub fn new(gnb_du_ue_f1ap_id: GnbDuUeF1apId, nr_cgi: NrCgi) -> Self {
        UeState {
            key: rand::thread_rng().gen::<u32>(),
            gnb_du_ue_f1ap_id,
            nr_cgi,
            gnb_cu_up_ue_e1ap_id: None,
            amf_ue_ngap_id: None,
        }
    }
}

impl SerDes for UeState {
    fn into_bytes(self) -> Result<Vec<u8>> {
        Ok(UeStateSerializable::try_from(self)?.write_to_vec()?)
    }
    fn from_bytes(v: &[u8]) -> Result<Self> {
        let s = UeStateSerializable::read_from_buffer(v)?;
        UeState::try_from(s)
    }
}

impl TryFrom<UeState> for UeStateSerializable {
    type Error = anyhow::Error;
    fn try_from(x: UeState) -> Result<Self> {
        Ok(UeStateSerializable {
            key: x.key,
            nr_cgi: Asn1Serdes::into_bytes(x.nr_cgi)?,
            gnb_du_ue_f1ap_id: x.gnb_du_ue_f1ap_id.0,
            amf_ue_ngap_id: x.amf_ue_ngap_id.map(|x| x.0),
            gnb_cu_up_ue_e1ap_id: x.gnb_cu_up_ue_e1ap_id.map(|x| x.0),
        })
    }
}

impl TryFrom<UeStateSerializable> for UeState {
    type Error = anyhow::Error;
    fn try_from(x: UeStateSerializable) -> Result<Self> {
        Ok(UeState {
            key: x.key,
            nr_cgi: Asn1Serdes::from_bytes(&x.nr_cgi)?,
            gnb_du_ue_f1ap_id: GnbDuUeF1apId(x.gnb_du_ue_f1ap_id),
            amf_ue_ngap_id: x.amf_ue_ngap_id.map(AmfUeNgapId),
            gnb_cu_up_ue_e1ap_id: x.gnb_cu_up_ue_e1ap_id.map(GnbCuUpUeE1apId),
        })
    }
}
