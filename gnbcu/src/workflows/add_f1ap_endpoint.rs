//! add_e1ap_endpoint - Adds an E1AP endpoint

use super::Workflow;
use crate::gnbcu_trait::Gnbcu;
use anyhow::Result;
use f1ap::{
    CpTransportLayerAddress, GnbCuConfigurationUpdate, GnbCuConfigurationUpdateProcedure,
    GnbCuTnlAssociationToAddItem, GnbCuTnlAssociationToAddList, TnlAssociationUsage, TransactionId,
    TransportLayerAddress,
};

impl<'a, G: Gnbcu> Workflow<'a, G> {
    pub async fn add_f1ap_endpoint(&self, f1ap_endpoint_ip_addr: &String) -> Result<()> {
        self.log_message("Add F1AP endpoint");

        let gnb_cu_configuration_update = GnbCuConfigurationUpdate {
            transaction_id: TransactionId(1), // TODO
            cells_to_be_activated_list: None,
            cells_to_be_deactivated_list: None,
            gnb_cu_tnl_association_to_add_list: Some(GnbCuTnlAssociationToAddList(vec![
                GnbCuTnlAssociationToAddItem {
                    tnl_association_transport_layer_address:
                        CpTransportLayerAddress::EndpointIpAddress(TransportLayerAddress(
                            net::ip_bits_from_string(f1ap_endpoint_ip_addr)?,
                        )),
                    tnl_association_usage: TnlAssociationUsage::Both,
                },
            ])),
            gnb_cu_tnl_association_to_remove_list: None,
            gnb_cu_tnl_association_to_update_list: None,
            cells_to_be_barred_list: None,
            protected_eutra_resources_list: None,
            neighbour_cell_information_list: None,
            transport_layer_address_info: None,
            ul_bh_non_up_traffic_mapping: None,
            bap_address: None,
        };

        self.log_message("<< GnbCuConfigurationUpdate");
        let _response = self
            .f1ap_request::<GnbCuConfigurationUpdateProcedure>(
                gnb_cu_configuration_update,
                self.logger,
            )
            .await?;
        self.log_message(">> GnbCuConfigurationUpdateAcknowledge");
        Ok(())
    }
}
