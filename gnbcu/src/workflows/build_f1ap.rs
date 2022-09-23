use crate::datastore::UeState;

use super::Gnbcu;
use bitvec::prelude::*;
use f1ap::{GnbCuUeF1apId, UeContextSetupRequest};
use ngap::*;

pub fn build_ue_context_setup_request<G: Gnbcu>(
    gnbcu: &G,
    _r: &InitialContextSetupRequest,
    ue: &UeState,
    rrc_container: Option<f1ap::RrcContainer>,
) -> UeContextSetupRequest {
    // TODO: derive and use frunk for the common ngap / f1ap structures seen here.

    UeContextSetupRequest {
        gnb_cu_ue_f1ap_id: GnbCuUeF1apId(ue.key),
        gnb_du_ue_f1ap_id: Some(ue.gnb_du_ue_f1ap_id.clone()),
        sp_cell_id: f1ap::NrCgi {
            plmn_identity: f1ap::PlmnIdentity(gnbcu.config().plmn.clone()),
            nr_cell_identity: f1ap::NrCellIdentity(bitvec![u8,Msb0;0;36]),
        },
        serv_cell_index: f1ap::ServCellIndex(0),
        sp_cell_ul_configured: None,
        cu_to_du_rrc_information: f1ap::CuToDuRrcInformation {
            cg_config_info: None,
            ue_capability_rat_container_list: None,
            meas_config: None,
        },
        candidate_sp_cell_list: None,
        drx_cycle: None,
        resource_coordination_transfer_container: None,
        s_cell_to_be_setup_list: None,
        srbs_to_be_setup_list: None,
        drbs_to_be_setup_list: None,
        inactivity_monitoring_request: None,
        rat_frequency_priority_information: None,
        rrc_container,
        masked_imeisv: None, // r.masked_imeisv,
        serving_plmn: None,
        gnb_du_ue_ambr_ul: None,
        rrc_delivery_status_request: None,
        resource_coordination_transfer_information: None,
        serving_cell_mo: None,
        new_gnb_cu_ue_f1ap_id: None,
        ran_ue_id: None,
        trace_activation: None,
        additional_rrm_priority_index: None,
        bh_channels_to_be_setup_list: None,
        configured_bap_address: None,
        nr_v2x_services_authorized: None, // r.nr_v2x_services_authorized,
        ltev2x_services_authorized: None, // r.ltev2x_services_authorized,
        nr_ue_sidelink_aggregate_maximum_bitrate: None, // r.nr_ue_sidelink_aggregate_maximum_bitrate,
        lte_ue_sidelink_aggregate_maximum_bitrate: None, // r.lte_ue_sidelink_aggregate_maximum_bitrate,
        pc5_link_ambr: None, // r.pc5_qos_parameters.and_then(|x| x.pc_5_link_aggregate_bit_rates),
        sl_drbs_to_be_setup_list: None,
        conditional_inter_du_mobility_information: None,
        management_based_mdt_plmn_list: None,
        serving_nid: None,
        f1c_transfer_path: None,
    }
}
