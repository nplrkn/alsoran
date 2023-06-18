//! gnb_cu_cp_configuration_update - where the control plane updates its configuration
//! mostly notably to order the CU-UP to add a TNLA.

use super::{GnbCuUp, Workflow};
use anyhow::bail;
use asn1_per::NonEmpty;
use e1ap::*;
use slog::warn;

impl<'a, G: GnbCuUp> Workflow<'a, G> {
    pub async fn gnb_cu_cp_configuration_update(
        &self,
        r: GnbCuCpConfigurationUpdate,
    ) -> Result<GnbCuCpConfigurationUpdateAcknowledge, GnbCuCpConfigurationUpdateFailure> {
        self.log_message("GnbCuCpConfigurationUpdate <<");
        let transaction_id = r.transaction_id;
        self.inner(r)
            .await
            .map(|x| {
                self.log_message("GnbCuCpConfigurationUpdateAcknowledge >>");
                x
            })
            .map_err(|e| {
                warn!(self.logger, "Error during configuration update - {e}");
                self.log_message("GnbCuCpConfigurationUpdateFailure >>");
                GnbCuCpConfigurationUpdateFailure {
                    transaction_id,
                    time_to_wait: None,
                    cause: Cause::RadioNetwork(CauseRadioNetwork::Unspecified),
                    criticality_diagnostics: None,
                }
            })
    }

    async fn inner(
        &self,
        r: GnbCuCpConfigurationUpdate,
    ) -> anyhow::Result<GnbCuCpConfigurationUpdateAcknowledge> {
        let Some(tnlas_to_add) = r.gnb_cu_cp_tnla_to_add_list else {
            bail!("Current support for GnbCuCpConfigurationUpdate is limited to adding TNLAs")
        };

        let mut added_tnlas = vec![];
        for tnla_to_add in tnlas_to_add.0 {
            let GnbCuCpTnlaToAddItem {
                tnl_association_transport_layer_address: CpTnlInformation::EndpointIpAddress(ip_address),
                ..
            } = tnla_to_add else {
                bail!("Port variant not supported on CpTnlInformation")
            };

            self.e1ap_connect(&ip_address.clone().try_into()?).await?;
            added_tnlas.push(GnbCuCpTnlaSetupItem {
                tnl_association_transport_layer_address: CpTnlInformation::EndpointIpAddress(
                    ip_address,
                ),
            })
        }
        let gnb_cu_cp_tnla_setup_list = NonEmpty::from_vec(added_tnlas).map(GnbCuCpTnlaSetupList);

        Ok(GnbCuCpConfigurationUpdateAcknowledge {
            transaction_id: r.transaction_id,
            criticality_diagnostics: None,
            gnb_cu_cp_tnla_setup_list,
            gnb_cu_cp_tnla_failed_to_setup_list: None,
            transport_layer_address_info: None,
        })
    }
}
