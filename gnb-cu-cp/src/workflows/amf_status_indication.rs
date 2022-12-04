//! amf_status_indication - AMF warns that some GUAMIs are going unavailable

use super::Workflow;
use crate::gnb_cu_cp::GnbCuCp;
use ngap::*;
use slog::info;

impl<'a, G: GnbCuCp> Workflow<'a, G> {
    pub async fn amf_status_indication(&self, i: AmfStatusIndication) {
        self.log_message("<< Amf Status Indication");
        for guami_item in i.unavailable_guami_list.0 {
            info!(self.logger, "GUAMI {} now unavailable", guami_item.guami);
        }
    }
}
