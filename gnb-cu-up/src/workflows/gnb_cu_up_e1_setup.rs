//! gnb_cu_up_e1_setup - the initial handshake that establishes an instance of the E1 reference point between CU-CP and CU-UP

use super::{GnbCuUp, Workflow};
use anyhow::Result;
use async_net::IpAddr;
use e1ap::*;
use slog::info;

impl<'a, G: GnbCuUp> Workflow<'a, G> {
    // E1 Setup Procedure
    // 1.    Connect to the CU-CP
    // 2.    E1 GnbCuUpE1SetupRequest >>
    // 3.    E1 GnbCuUpE1SetupResponse <<
    pub async fn gnb_cu_up_e1_setup(
        &self,
        cu_cp_address: &IpAddr,
        gnb_cu_up_id: u64,
        supported_plmns: SupportedPlmnsList,
    ) -> Result<()> {
        // Connect to the CU-CP.
        self.e1ap_connect(cu_cp_address).await?;

        // Send E1 setup request.
        let e1_setup_request = GnbCuUpE1SetupRequest {
            transaction_id: TransactionId(1),
            gnb_cu_up_id: GnbCuUpId(gnb_cu_up_id),
            gnb_cu_up_name: Some(GnbCuUpName("Alsoran CU-UP".to_string())),
            cn_support: CnSupport::C5gc,
            supported_plmns,
            gnb_cu_up_capacity: None,
            transport_layer_address_info: None,
            extended_gnb_cu_up_name: None,
        };
        self.log_message("GnbCuUpE1SetupRequest >>");
        let response = self
            .e1ap_request::<GnbCuUpE1SetupProcedure>(e1_setup_request, self.logger)
            .await?;
        self.log_message("GnbCuUpE1SetupResponse <<");
        info!(
            self.logger,
            "E1AP interface initialized with {:?}", response.gnb_cu_cp_name
        );

        Ok(())
    }
}
