use anyhow::Result;
use net::AperSerde;
use pdcp::PdcpPdu;
use rrc::*;

pub fn make_rrc_container(rrc: DlDcchMessage) -> Result<f1ap::RrcContainer> {
    let rrc_bytes = rrc.into_bytes()?;
    Ok(f1ap::RrcContainer(PdcpPdu::encode(&rrc_bytes).into()))
}

pub fn build_rrc_security_mode_command(
    rrc_transaction_identifier: u8,
) -> Result<f1ap::RrcContainer> {
    let rrc_transaction_identifier = RrcTransactionIdentifier(rrc_transaction_identifier);

    make_rrc_container(DlDcchMessage {
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
    })
}

pub fn build_rrc_reconfiguration(
    rrc_transaction_identifier: u8,
    nas_messages: Option<Vec<Vec<u8>>>,
) -> Result<f1ap::RrcContainer> {
    let rrc_transaction_identifier = RrcTransactionIdentifier(rrc_transaction_identifier);
    let dedicated_nas_message_list =
        nas_messages.map(|x| (x.into_iter().map(|x| DedicatedNasMessage(x)).collect()));

    make_rrc_container(DlDcchMessage {
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
                    dedicated_nas_message_list,
                    master_key_update: None,
                    dedicated_sib1_delivery: None,
                    dedicated_system_information_delivery: None,
                    other_config: None,
                    non_critical_extension: None,
                }),
            }),
        })),
    })
}