//! f1_setup - the initial handshake that establishes an instance of the F1 reference point between GNB-CU and GNB-DU

use super::Workflow;
use crate::gnb_cu_cp::GnbCuCp;
use anyhow::Result;
use f1ap::*;
use net::{RequestError, ResponseAction};

impl<'a, G: GnbCuCp> Workflow<'a, G> {
    // F1 Setup Procedure
    // 1.    F1ap GnbDuConfigurationUpdate >>
    // 2.    F1ap GnbDuConfigurationUpdateAcknowledge <<
    pub async fn gnb_du_configuration_update(
        &self,
        r: GnbDuConfigurationUpdate,
    ) -> Result<
        ResponseAction<GnbDuConfigurationUpdateAcknowledge>,
        RequestError<GnbDuConfigurationUpdateFailure>,
    > {
        self.log_message(">> GnbDuConfigurationUpdate");

        if let Some(_) = r.served_cells_to_add_list {
            self.log_message_error("Served cells to add present on GnbDuConfigurationUpdate but not implemented and ignored")
        }
        if let Some(_) = r.served_cells_to_modify_list {
            self.log_message_error("Served cells to modify present on GnbDuConfigurationUpdate but not implemented and ignored")
        }
        if let Some(_) = r.served_cells_to_delete_list {
            self.log_message_error("Served cells to delete present on GnbDuConfigurationUpdate but not implemented and ignored")
        }
        if let Some(_) = r.gnb_du_tnl_association_to_remove_list {
            self.log_message_error("Tnl association to delete present on GnbDuConfigurationUpdate but not implemented and ignored")
        }

        self.log_message("<< GnbDuConfigurationUpdateAcknowledge");
        Ok((
            GnbDuConfigurationUpdateAcknowledge {
                transaction_id: r.transaction_id,
                cells_to_be_activated_list: None,
                criticality_diagnostics: None,
                cells_to_be_deactivated_list: None,
                transport_layer_address_info: None,
                ul_bh_non_up_traffic_mapping: None,
                bap_address: None,
            },
            None,
        ))
    }
}
