//! f1_setup - the initial handshake that establishes an instance of the F1 reference point between GNB-CU and GNB-DU

use super::Workflow;
use crate::gnbcu_trait::Gnbcu;
use anyhow::Result;
use bitvec::prelude::*;
use f1ap::*;
use net::RequestError;
use slog::info;

impl<'a, G: Gnbcu> Workflow<'a, G> {
    // F1 Setup Procedure
    // 1.    F1ap F1SetupRequest >>
    // 2.    F1ap F1SetupResponse <<
    pub async fn f1_setup(
        &self,
        r: F1SetupRequest,
    ) -> Result<F1SetupResponse, RequestError<F1SetupFailure>> {
        self.log_message(">> F1SetupRequest");
        info!(
            self.logger,
            "F1AP interface initialized with {:?}", r.gnb_du_id
        );

        self.log_message("<< F1SetupResponse");
        Ok(F1SetupResponse {
            transaction_id: r.transaction_id,
            gnb_cu_rrc_version: RrcVersion {
                latest_rrc_version: bitvec![u8, Msb0;0, 0, 0],
            },
            gnb_cu_name: self.gnbcu.config().clone().name.map(|x| GnbCuName(x)),
            cells_to_be_activated_list: None,
            transport_layer_address_info: None,
            ul_bh_non_up_traffic_mapping: None,
            bap_address: None,
            extended_gnb_du_name: None,
        })
    }
}
