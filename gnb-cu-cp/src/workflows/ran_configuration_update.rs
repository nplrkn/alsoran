//! ran_configuration_update - the first transaction sent by the gNB over a new TNLA within an existing NGAP interface instance

use super::{GnbCuCp, Workflow};
use anyhow::Result;
use ngap::*;

impl<'a, G: GnbCuCp> Workflow<'a, G> {
    // Ran Configuration Update Procedure
    // 1.    Connect to the AMF
    // 2.    Ngap RanConfigurationUpdate >>
    // 3.    Ngap RanConfigurationUpdateAcknowledge <<
    pub async fn ran_configuration_update(&self, amf_ip_address: &str) -> Result<()> {
        // Connect to the AMF
        self.gnb_cu_cp.ngap_connect(amf_ip_address).await?;

        // This uses the default expected values of free5GC.

        // TS38.413, 8.7.2.2.
        // If the Global RAN Node ID IE is included in the RAN CONFIGURATION UPDATE message,
        // the AMF shall associate the TNLA to the NG-C interface instance using the Global RAN Node ID.
        let ran_configuration_update = RanConfigurationUpdate {
            ran_node_name: None,
            supported_ta_list: None,
            default_paging_drx: None,
            global_ran_node_id: Some(super::build_ngap::build_global_ran_node_id(self.gnb_cu_cp)),
            ngran_tnl_association_to_remove_list: None,
            nb_iot_default_paging_drx: None,
            extended_ran_node_name: None,
        };
        self.log_message("RanConfigurationUpdate >>");
        let _response = self
            .ngap_request::<RanConfigurationUpdateProcedure>(ran_configuration_update, self.logger)
            .await?;
        self.log_message("RanConfigurationUpdateAcknowledge <<");

        // Associate this TNLA with the NGAP interface instance.
        // self.associate_connection();

        Ok(())
    }
}
