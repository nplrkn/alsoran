use crate::{Gnbcu, UeContext};
use anyhow::Result;
use bitvec::prelude::*;
use f1ap::{GnbCuUeF1apId, GnbDuUeF1apId, UeContextSetupProcedure, UeContextSetupRequest};
use net::{AperSerde, RequestError, RequestProvider, Stack};
use ngap::*;
use pdcp::PdcpPdu;
use rrc::*;
use slog::{debug, Logger};

// Carry out initial context setup procedure.
//
// 1.    Ngap Initial Context Setup Request + Nas <-
// 2. <- F1ap Ue Context Setup Request + Rrc Security Mode Command
// 3. -> F1ap Ue Context Setup Response
// 4. -> Rrc Security Mode Complete
// 5. <- Rrc Reconfiguration + Nas
// 6. -> Rrc Reconfiguration Complete
// 7.    Ngap Initial Context Setup Response ->
pub async fn initial_context_setup(
    gnbcu: &Gnbcu,
    r: InitialContextSetupRequest,
    logger: &Logger,
) -> Result<InitialContextSetupResponse, RequestError<InitialContextSetupFailure>> {
    debug!(&logger, "Initial Context Setup + Nas << ");

    // Todo - this should be Result<InitialContextSetupResponse, Cause>, and the caller
    // should create the InitialContextSetupFailure.

    // Todo - retrieve UE context by ran_ue_ngap_id.
    let ue = UeContext {
        gnb_du_ue_f1ap_id: GnbDuUeF1apId(1),
        gnb_cu_ue_f1ap_id: GnbCuUeF1apId(1),
    };

    // Build Security Mode command and wrap it in an RrcContainer.
    let rrc_transaction = gnbcu.new_rrc_transaction(&ue).await;
    let rrc_security_mode_command = build_rrc_security_mode_command(RrcTransactionIdentifier(2));
    let rrc_container = Some(make_rrc_container(rrc_security_mode_command)?);

    // Build Ue Context Setup request and include the Rrc security mode command.
    let ue_context_setup_request = build_ue_context_setup_request(&r, rrc_container);

    // Send to GNB-DU and get back the response to the (outer) UE Context Setup.
    debug!(&logger, "<< Ue Context Setup + Rrc Security Mode Command");
    let _ue_context_setup_response = <Stack as RequestProvider<UeContextSetupProcedure>>::request(
        &gnbcu.f1ap,
        ue_context_setup_request,
        &logger,
    )
    .await
    .map_err(|_| RequestError::UnsuccessfulOutcome(build_initial_context_setup_failure()))?;
    debug!(&logger, ">> Ue Context Setup response");

    // Also get back the response from the UE to the (inner) Security Mode Command.
    let _rrc_security_mode_complete = rrc_transaction.recv().await?;
    debug!(&logger, ">> Rrc Security Mode Complete");

    // Build Rrc Reconfiguration including the Nas message from earlier.
    let rrc_transaction = gnbcu.new_rrc_transaction(&ue).await;
    let rrc_reconfiguration = build_rrc_reconfiguration(
        RrcTransactionIdentifier(3),
        r.nas_pdu.map(|x| DedicatedNasMessage(x.0)),
    );
    let rrc_container = make_rrc_container(rrc_reconfiguration)?;

    // Send to the UE and get back the response.
    debug!(&logger, "<< Rrc Reconfiguration");
    gnbcu.send_rrc_to_ue(ue, rrc_container, logger).await;
    let _rrc_reconfiguration_complete: UlDcchMessage = rrc_transaction.recv().await?;
    debug!(&logger, ">> Rrc Reconfiguration Complete");

    // Reply to the AMF.
    debug!(&logger, "Initial Context Setup response >>");
    Ok(InitialContextSetupResponse {
        amf_ue_ngap_id: r.amf_ue_ngap_id,
        ran_ue_ngap_id: RanUeNgapId(1),
        pdu_session_resource_setup_list_cxt_res: None,
        pdu_session_resource_failed_to_setup_list_cxt_res: None,
        criticality_diagnostics: None,
    })
}

fn make_rrc_container(rrc: DlDcchMessage) -> Result<f1ap::RrcContainer> {
    let rrc_bytes = rrc.into_bytes()?;
    Ok(f1ap::RrcContainer(PdcpPdu::encode(&rrc_bytes).bytes()))
}

fn build_rrc_security_mode_command(
    rrc_transaction_identifier: RrcTransactionIdentifier,
) -> DlDcchMessage {
    DlDcchMessage {
        message: DlDcchMessageType::C1(C1_2::SecurityModeCommand(rrc::SecurityModeCommand {
            rrc_transaction_identifier,
            critical_extensions: CriticalExtensions26::SecurityModeCommand(
                SecurityModeCommandIEs {
                    security_config_smc: SecurityConfigSmc {
                        security_algorithm_config: SecurityAlgorithmConfig {
                            ciphering_algorithm: CipheringAlgorithm::Nea0,
                            integrity_prot_algorithm: None,
                        },
                    },
                    late_non_critical_extension: None,
                },
            ),
        })),
    }
}

fn build_ue_context_setup_request(
    _r: &InitialContextSetupRequest,
    rrc_container: Option<f1ap::RrcContainer>,
) -> UeContextSetupRequest {
    // TODO: derive and use frunk for the common ngap / f1ap structures seen here.

    UeContextSetupRequest {
        gnb_cu_ue_f1ap_id: GnbCuUeF1apId(1),
        gnb_du_ue_f1ap_id: Some(GnbDuUeF1apId(1)),
        sp_cell_id: f1ap::NrCgi {
            plmn_identity: f1ap::PlmnIdentity(vec![0, 1, 2]),
            nr_cell_identity: f1ap::NrCellIdentity(bitvec![Msb0,u8;0;36]),
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

fn build_rrc_reconfiguration(
    rrc_transaction_identifier: RrcTransactionIdentifier,
    nas: Option<DedicatedNasMessage>,
) -> DlDcchMessage {
    DlDcchMessage {
        message: DlDcchMessageType::C1(C1_2::RrcReconfiguration(rrc::RrcReconfiguration {
            rrc_transaction_identifier,
            critical_extensions: CriticalExtensions15::RrcReconfiguration(RrcReconfigurationIEs {
                radio_bearer_config: None,
                secondary_cell_group: None,
                meas_config: None,
                late_non_critical_extension: None,
                non_critical_extension: Some(RrcReconfigurationV1530IEs {
                    master_cell_group: None,
                    full_config: None,
                    dedicated_nas_message_list: nas.map(|x| vec![x]),
                    master_key_update: None,
                    dedicated_sib1_delivery: None,
                    dedicated_system_information_delivery: None,
                    other_config: None,
                    non_critical_extension: None,
                }),
            }),
        })),
    }
}

fn build_initial_context_setup_failure() -> InitialContextSetupFailure {
    todo!()
}