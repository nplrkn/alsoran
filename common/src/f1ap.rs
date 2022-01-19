use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum F1apPdu {
    InitiatingMessage,
    SuccessfulOutcome,
    UnsuccessfulOutcome,
}
