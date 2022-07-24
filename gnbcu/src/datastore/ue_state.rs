use f1ap::GnbDuUeF1apId;
use ngap::AmfUeNgapId;

#[derive(Clone, Debug)]
pub struct UeState {
    pub key: u32,
    pub gnb_du_ue_f1ap_id: GnbDuUeF1apId,
    pub amf_ue_ngap_id: Option<AmfUeNgapId>,
}
