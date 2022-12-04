//! add_e1ap_endpoint - Adds an E1AP endpoint

use super::Workflow;
use crate::gnb_cu_cp::GnbCuCp;
use anyhow::Result;
use e1ap::{
    CpTnlInformation, GnbCuCpConfigurationUpdate, GnbCuCpConfigurationUpdateProcedure,
    GnbCuCpTnlaToAddItem, GnbCuCpTnlaToAddList, TnlAssociationUsage, TransactionId,
    TransportLayerAddress,
};

impl<'a, G: GnbCuCp> Workflow<'a, G> {
    pub async fn gnb_cu_cp_configuration_update(
        &self,
        e1ap_endpoint_ip_addr: &String,
    ) -> Result<i32> {
        let gnb_cu_cp_configuration_update = GnbCuCpConfigurationUpdate {
            transaction_id: TransactionId(1), // TODO
            gnb_cu_cp_name: None,
            gnb_cu_cp_tnla_to_add_list: Some(GnbCuCpTnlaToAddList(vec![GnbCuCpTnlaToAddItem {
                tnl_association_transport_layer_address: CpTnlInformation::EndpointIpAddress(
                    TransportLayerAddress(net::ip_bits_from_string(e1ap_endpoint_ip_addr)?),
                ),
                tnl_association_usage: TnlAssociationUsage::Both,
            }])),
            gnb_cu_cp_tnla_to_remove_list: None,
            gnb_cu_cp_tnla_to_update_list: None,
            transport_layer_address_info: None,
            extended_gnb_cu_cp_name: None,
        };

        self.log_message("<< GnbCuCpConfigurationUpdate");
        let _response = self
            .e1ap_request::<GnbCuCpConfigurationUpdateProcedure>(
                gnb_cu_cp_configuration_update,
                self.logger,
            )
            .await?;
        self.log_message(">> GnbCuCpConfigurationUpdateAcknowledge");

        // Associate this TNLA with the E1AP interface instance.
        let revision_number = self.associate_connection();

        Ok(revision_number)
    }
}
