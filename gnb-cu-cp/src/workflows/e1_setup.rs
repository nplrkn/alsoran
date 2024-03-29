//! e1_setup - the initial handshake that establishes an instance of the E1 reference point between GNB-CU and GNB-DU

use super::Workflow;
use crate::gnb_cu_cp::GnbCuCp;
use anyhow::Result;
use e1ap::*;
use net::{RequestError, ResponseAction};
use slog::info;

impl<'a, G: GnbCuCp> Workflow<'a, G> {
    // E1 Setup Procedure
    // 1.    E1ap GnbCuUpE1SetupRequest >>
    // 2.    E1ap GnbCuUpE1SetupResponse <<
    // Then update coordinator as a follow-on task.
    pub async fn e1_setup(
        &self,
        r: GnbCuUpE1SetupRequest,
    ) -> Result<ResponseAction<GnbCuUpE1SetupResponse>, RequestError<GnbCuUpE1SetupFailure>> {
        self.log_message(">> GnbCuUpE1SetupRequest");
        info!(
            self.logger,
            "E1AP interface initialized with {:?}", r.gnb_cu_up_id
        );

        // Associate this TNLA with the E1AP interface instance.
        let coordinator_notify = self.associate_connection();

        self.log_message("<< GnbCuUpE1SetupResponse");
        Ok((
            GnbCuUpE1SetupResponse {
                transaction_id: r.transaction_id,
                gnb_cu_cp_name: self.gnb_cu_cp.config().clone().name.map(GnbCuCpName),
                transport_layer_address_info: None,
                extended_gnb_cu_cp_name: None,
            },
            Some(coordinator_notify),
        ))
    }
}
