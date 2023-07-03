//! build_rrc - construction of RRC messages

use anyhow::Result;
use asn1_per::{nonempty, NonEmpty};
use net::*;
use pdcp::PdcpPdu;
use rrc::*;

pub fn make_pdcp_encapsulated_rrc_container<T: SerDes>(rrc: T) -> Result<f1ap::RrcContainer> {
    let rrc_bytes = rrc.into_bytes()?;
    Ok(f1ap::RrcContainer(PdcpPdu::encode(&rrc_bytes).into()))
}

pub fn build_rrc_setup(rrc_transaction_identifier: u8) -> Result<f1ap::RrcContainer> {
    let message_bytes = DlCcchMessage {
        message: DlCcchMessageType::C1(C1_1::RrcSetup(RrcSetup {
            rrc_transaction_identifier: RrcTransactionIdentifier(rrc_transaction_identifier),
            critical_extensions: CriticalExtensions21::RrcSetup(RrcSetupIEs {
                radio_bearer_config: RadioBearerConfig {
                    srb_to_add_mod_list: None,
                    srb_3_to_release: None,
                    drb_to_add_mod_list: None,
                    drb_to_release_list: None,
                    security_config: None,
                },
                master_cell_group: vec![],
                late_non_critical_extension: None,
            }),
        })),
    }
    .into_bytes()?;

    // As per TS38.473, 9.2.3.2, a DL-CCCH-Message is _not_ encoded in a PDCP PDU (unlike a DL-DCCH-Message).
    // This is why we don't use make_pdcp_encapsulted_rrc_container().
    Ok(f1ap::RrcContainer(message_bytes))
}

pub fn build_rrc_security_mode_command(
    rrc_transaction_identifier: u8,
) -> Result<f1ap::RrcContainer> {
    let rrc_transaction_identifier = RrcTransactionIdentifier(rrc_transaction_identifier);

    make_pdcp_encapsulated_rrc_container(DlDcchMessage {
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

pub fn build_rrc_dl_information_transfer(
    rrc_transaction_identifier: u8,
    dedicated_nas_message: DedicatedNasMessage,
) -> Result<f1ap::RrcContainer> {
    make_pdcp_encapsulated_rrc_container(DlDcchMessage {
        message: DlDcchMessageType::C1(C1_2::DlInformationTransfer(DlInformationTransfer {
            rrc_transaction_identifier: RrcTransactionIdentifier(rrc_transaction_identifier),
            critical_extensions: CriticalExtensions4::DlInformationTransfer(
                DlInformationTransferIEs {
                    dedicated_nas_message: Some(dedicated_nas_message),
                    late_non_critical_extension: None,
                    non_critical_extension: None,
                },
            ),
        })),
    })
}

pub fn build_rrc_reconfiguration(
    rrc_transaction_identifier: u8,
    nas_messages: Option<NonEmpty<Vec<u8>>>,
    cell_group_config: Vec<u8>,
) -> Result<f1ap::RrcContainer> {
    let dedicated_nas_message_list = nas_messages.map(|x| (x.map(DedicatedNasMessage)));

    // TODO - lots of hardcoding here

    make_pdcp_encapsulated_rrc_container(DlDcchMessage {
        message: DlDcchMessageType::C1(C1_2::RrcReconfiguration(rrc::RrcReconfiguration {
            rrc_transaction_identifier: RrcTransactionIdentifier(rrc_transaction_identifier),
            critical_extensions: CriticalExtensions15::RrcReconfiguration(RrcReconfigurationIEs {
                radio_bearer_config: Some(RadioBearerConfig {
                    srb_to_add_mod_list: None,
                    srb_3_to_release: None,
                    drb_to_add_mod_list: Some(DrbToAddModList(nonempty![DrbToAddMod {
                        cn_association: Some(CnAssociation::SdapConfig(SdapConfig {
                            pdu_session: PduSessionId(1),
                            sdap_header_dl: SdapHeaderDl::Present,
                            sdap_header_ul: SdapHeaderUl::Present,
                            default_drb: true,
                            mapped_qos_flows_to_add: None,
                            mapped_qos_flows_to_release: None
                        })),
                        drb_identity: DrbIdentity(1),
                        reestablish_pdcp: None,
                        recover_pdcp: None,
                        pdcp_config: Some(PdcpConfig {
                            drb: Some(Drb {
                                discard_timer: Some(DiscardTimer::Ms10),
                                pdcp_sn_size_ul: Some(PdcpSnSizeUl::Len12bits),
                                pdcp_sn_size_dl: Some(PdcpSnSizeDl::Len12bits),
                                header_compression: HeaderCompression::NotUsed,
                                integrity_protection: None,
                                status_report_required: None,
                                out_of_order_delivery: None
                            }),
                            more_than_one_rlc: None,
                            t_reordering: None
                        })
                    }])),
                    drb_to_release_list: None,
                    security_config: None,
                }),
                secondary_cell_group: None,
                meas_config: None,
                late_non_critical_extension: None,
                non_critical_extension: Some(RrcReconfigurationV1530IEs {
                    master_cell_group: Some(cell_group_config),
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
