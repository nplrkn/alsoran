use f1ap::{GnbCuUeF1apId, GnbDuUeF1apId};

pub struct UeContext {
    pub gnb_du_ue_f1ap_id: GnbDuUeF1apId,
    pub gnb_cu_ue_f1ap_id: GnbCuUeF1apId,
}
