use f1ap::GnbDuUeF1apId;

#[derive(Clone, Debug)]
pub struct UeState {
    pub key: u64,
    pub gnb_du_ue_f1ap_id: GnbDuUeF1apId,
}
