use f1ap::{GnbCuUeF1apId, GnbDuUeF1apId};

#[derive(Clone, Debug)]
pub struct UeState {
    pub gnb_du_ue_f1ap_id: GnbDuUeF1apId,
    pub gnb_cu_ue_f1ap_id: GnbCuUeF1apId,
}
