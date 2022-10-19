//! add_e1ap_endpoint - Adds an E1AP endpoint

use super::Workflow;
use crate::gnbcu_trait::Gnbcu;

impl<'a, G: Gnbcu> Workflow<'a, G> {
    pub async fn add_e1ap_endpoint(&self) {
        // self.log_message("<< Amf Status Indication");
        // for guami_item in i.unavailable_guami_list.0 {
        //     info!(self.logger, "GUAMI {} now unavailable", guami_item.guami);
        // }
        todo!()
    }
}
