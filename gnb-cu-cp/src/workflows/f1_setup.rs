//! f1_setup - the initial handshake that establishes an instance of the F1 reference point between GNB-CU and GNB-DU

use super::Workflow;
use crate::gnb_cu_cp::GnbCuCp;
use anyhow::Result;
use bitvec::prelude::*;
use f1ap::*;
use net::{RequestError, ResponseAction};
use slog::info;

impl<'a, G: GnbCuCp> Workflow<'a, G> {
    // F1 Setup Procedure
    // 1.    F1ap F1SetupRequest >>
    // 2.    F1ap F1SetupResponse <<
    pub async fn f1_setup(
        &self,
        r: F1SetupRequest,
    ) -> Result<ResponseAction<F1SetupResponse>, RequestError<F1SetupFailure>> {
        self.log_message(">> F1SetupRequest");
        info!(
            self.logger,
            "F1AP interface initialized with {:?}", r.gnb_du_id
        );

        let coordinator_notify = self.associate_connection();

        // Activate all served cells in the setup response.
        // TODO: store information about served cells for use later.
        let cells_to_be_activated_list = r
            .gnb_du_served_cells_list
            .map(|cells| CellsToBeActivatedList(cells.0.map(served_cell_to_activated)));

        self.log_message("<< F1SetupResponse");
        Ok((
            F1SetupResponse {
                transaction_id: r.transaction_id,
                gnb_cu_rrc_version: RrcVersion {
                    latest_rrc_version: bitvec![u8, Msb0;0, 0, 0],
                    latest_rrc_version_enhanced: None,
                },
                gnb_cu_name: self.gnb_cu_cp.config().clone().name.map(GnbCuName),
                cells_to_be_activated_list,
                transport_layer_address_info: None,
                ul_bh_non_up_traffic_mapping: None,
                bap_address: None,
                extended_gnb_du_name: None,
            },
            // Notify the coordinator as a follow on action after sending the response
            Some(coordinator_notify),
        ))
    }
}

fn served_cell_to_activated(served_cell: GnbDuServedCellsItem) -> CellsToBeActivatedListItem {
    let served_cell_information = &served_cell.served_cell_information;

    CellsToBeActivatedListItem {
        nr_cgi: served_cell_information.nr_cgi.clone(),
        nr_pci: None,
        gnb_cu_system_information: None,
        // gnb_cu_system_information: Some(GnbCuSystemInformation {
        //     sibtypetobeupdatedlist: (),
        //     system_information_area_id: (),
        // }),
        available_plmn_list: None,
        extended_available_plmn_list: None,
        iab_info_iab_donor_cu: None,
        available_snpn_id_list: None,
    }
}
