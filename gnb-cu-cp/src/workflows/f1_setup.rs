//! f1_setup - the initial handshake that establishes an instance of the F1 reference point between GNB-CU and GNB-DU

use super::Workflow;
use crate::gnb_cu_cp::GnbCuCp;
use anyhow::Result;
use asn1_per::{nonempty, SerDes};
use bitvec::prelude::*;
use f1ap::*;
use net::{RequestError, ResponseAction};
use rrc::{
    BcchDlSchMessage, BcchDlSchMessageType, CellReselectionInfoCommon, CellReselectionPriority,
    CellReselectionServingFreqInfo, CriticalExtensions30, IntraFreqCellReselectionInfo, QHyst,
    QRxLevMin, SibTypeAndInfo, SystemInformation, SystemInformationIEs, C1,
};
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
        let sib2_message = build_sib2_message().into_bytes()?;
        let cells_to_be_activated_list = r.gnb_du_served_cells_list.map(|cells| {
            CellsToBeActivatedList(
                cells
                    .0
                    .map(|x| served_cell_to_activated(x, sib2_message.clone())),
            )
        });

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

fn build_sib2_message() -> BcchDlSchMessage {
    let sib2 = rrc::Sib2 {
        cell_reselection_info_common: CellReselectionInfoCommon {
            nrof_ss_blocks_to_average: None,
            abs_thresh_ss_blocks_consolidation: None,
            range_to_best_cell: None,
            q_hyst: QHyst::Db1,
            speed_state_reselection_pars: None,
        },
        cell_reselection_serving_freq_info: CellReselectionServingFreqInfo {
            s_non_intra_search_p: None,
            s_non_intra_search_q: None,
            thresh_serving_low_p: rrc::ReselectionThreshold(2),
            thresh_serving_low_q: None,
            cell_reselection_priority: CellReselectionPriority(2),
            cell_reselection_sub_priority: None,
        },
        intra_freq_cell_reselection_info: IntraFreqCellReselectionInfo {
            q_rx_lev_min: QRxLevMin(-50),
            q_rx_lev_min_sul: None,
            q_qual_min: None,
            s_intra_search_p: rrc::ReselectionThreshold(2),
            s_intra_search_q: None,
            t_reselection_nr: rrc::TReselection(2),
            frequency_band_list: None,
            frequency_band_list_sul: None,
            p_max: None,
            smtc: None,
            ss_rssi_measurement: None,
            ssb_to_measure: None,
            derive_ssb_index_from_cell: true,
        },
    };
    rrc::BcchDlSchMessage {
        message: BcchDlSchMessageType::C1(C1::SystemInformation(SystemInformation {
            critical_extensions: CriticalExtensions30::SystemInformation(SystemInformationIEs {
                sib_type_and_info: nonempty![SibTypeAndInfo::Sib2(sib2)],
                late_non_critical_extension: None,
            }),
        })),
    }
}

fn served_cell_to_activated(
    served_cell: GnbDuServedCellsItem,
    sib_2: Vec<u8>,
) -> CellsToBeActivatedListItem {
    let served_cell_information = &served_cell.served_cell_information;
    let nr_pci = Some(served_cell_information.nr_pci);

    CellsToBeActivatedListItem {
        nr_cgi: served_cell_information.nr_cgi.clone(),
        nr_pci,
        gnb_cu_system_information: Some(GnbCuSystemInformation {
            sib_type_to_be_updated_list: nonempty![SibTypeToBeUpdatedListItem {
                sib_type: 2,
                sib_message: sib_2,
                value_tag: 0,
                area_scope: None
            }],
            system_information_area_id: None,
        }),
        available_plmn_list: None,
        extended_available_plmn_list: None,
        iab_info_iab_donor_cu: None,
        available_snpn_id_list: None,
    }
}
