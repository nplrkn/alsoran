use super::f1ap_ies::*;
use pdu_convert_derive::PduConvert;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum F1apPdu {
    F1SetupRequest(F1SetupRequest),
    F1SetupResponse(F1SetupResponse),
}

#[derive(Serialize, Deserialize, PduConvert, Clone, Debug)]
pub struct F1SetupRequest {
    pub transaction_id: TransactionId,
    pub gnb_du_id: GnbDuId,
    pub gnb_du_rrc_version: RrcVersion,
}

#[derive(Serialize, Deserialize, PduConvert, Clone, Debug)]
pub struct F1SetupResponse {
    pub transaction_id: TransactionId,
    pub gnb_cu_rrc_version: RrcVersion,
}
