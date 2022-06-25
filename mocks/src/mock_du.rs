use anyhow::{anyhow, Result};
use async_channel::{Receiver, Sender};
use async_trait::async_trait;
use bitvec::prelude::*;
use f1ap::*;
use net::{AperSerde, Indication, Message, TransportProvider};
use net::{SctpTransportProvider, TnlaEvent, TnlaEventHandler};
use pdcp::PdcpPdu;
use rrc::*;
use slog::{debug, info, o, Logger};
use stop_token::{StopSource, StopToken};

// TS38.472, section 7 - the Payload Protocol Identifier (ppid) assigned by IANA to be used by SCTP
// for the application layer protocol F1AP is 62
const F1AP_SCTP_PPID: u32 = 62;

// TODO make generic and commonize with MockAmf?
#[derive(Clone)]
pub struct MockDu {
    pub stop_token: StopToken,
    pub receiver: Receiver<Option<F1apPdu>>,
    pub sender: SctpTransportProvider,
    internal_sender: Sender<Option<F1apPdu>>,
    logger: Logger,
}

impl MockDu {
    pub async fn new(logger: &Logger) -> (MockDu, StopSource) {
        let logger = logger.new(o!("du" => 1));
        let (internal_sender, receiver) = async_channel::unbounded();
        let stop_source = StopSource::new();
        let sender = SctpTransportProvider::new(F1AP_SCTP_PPID);

        (
            MockDu {
                stop_token: stop_source.token(),
                receiver,
                sender,
                internal_sender,
                logger,
            },
            stop_source,
        )
    }

    pub async fn establish_connection(&self, connect_addr_string: String) -> Result<()> {
        let _task = self
            .sender
            .clone()
            .maintain_connection(
                connect_addr_string,
                self.stop_token.clone(),
                self.clone(),
                self.logger.clone(),
            )
            .await?;

        // Wait for the connection to be accepted.
        debug!(self.logger, "Wait for connection to be accepted by CU");
        match self.receiver.recv().await? {
            None => {
                info!(self.logger, "Successful connection establishment to CU");
                Ok(())
            }
            Some(_) => Err(anyhow!("Unexpectedly received a message")),
        }
    }

    /// Receive an F1apPdu from the GNB-CU, with a 0.5s timeout.
    async fn recv(&self) -> F1apPdu {
        async_std::future::timeout(std::time::Duration::from_millis(500), self.receiver.recv())
            .await
            .unwrap()
            .unwrap()
            .unwrap()
    }

    pub async fn perform_f1_setup(&self) -> Result<()> {
        let pdu =
            f1ap::F1apPdu::InitiatingMessage(InitiatingMessage::F1SetupRequest(F1SetupRequest {
                transaction_id: TransactionId(0),
                gnb_du_id: GnbDuId(123),
                gnb_du_rrc_version: RrcVersion {
                    latest_rrc_version: bitvec![Msb0, u8;0, 0, 0],
                },
                gnb_du_name: None,
                gnb_du_served_cells_list: None,
                transport_layer_address_info: None,
                bap_address: None,
                extended_gnb_cu_name: None,
            }));
        info!(self.logger, "Wait for F1 Setup response from GNB");
        self.sender
            .send_message(pdu.into_bytes()?, &self.logger)
            .await?;

        let _response = self.recv().await;
        info!(self.logger, "Got response from CU");
        Ok(())
    }

    pub async fn perform_rrc_setup(&self, nas_message: Vec<u8>, logger: &Logger) -> Result<()> {
        // Build RRC Setup Request
        let rrc_setup_request = UlCcchMessage {
            message: UlCcchMessageType::C1(C1_4::RrcSetupRequest(RrcSetupRequest {
                rrc_setup_request: RrcSetupRequestIEs {
                    ue_identity: InitialUeIdentity::Ng5gSTmsiPart1(bitvec![Msb0, u8; 0;39]),
                    establishment_cause: EstablishmentCause::MtAccess,
                    spare: bitvec![Msb0, u8;0;1],
                },
            })),
        }
        .into_bytes()?;

        let du_to_cu_rrc_container = Some(make_du_to_cu_rrc_container());

        // Wrap them in an F1 Initial UL Rrc Message Transfer.
        let f1_indication =
            InitialUlRrcMessageTransferProcedure::encode_request(InitialUlRrcMessageTransfer {
                gnb_du_ue_f1ap_id: GnbDuUeF1apId(1),
                nr_cgi: NrCgi {
                    plmn_identity: PlmnIdentity(vec![0, 1, 2]),
                    nr_cell_identity: NrCellIdentity(bitvec![Msb0,u8;0;36]),
                },
                c_rnti: CRnti(14),
                rrc_container: RrcContainer(rrc_setup_request),
                du_to_cu_rrc_container,
                sul_access_indication: None,
                transaction_id: TransactionId(1),
                ran_ue_id: None,
                rrc_container_rrc_setup_complete: None,
            })?;

        info!(
            &logger,
            "DU sends InitialUlRrcMessageTransfer containing RrcSetupRequest"
        );

        self.sender.send_message(f1_indication, logger).await?;

        // Receive DL Rrc Message Transfer and extract RRC Setup
        let dl_rrc_message_transfer = match self.recv().await {
            F1apPdu::InitiatingMessage(InitiatingMessage::DlRrcMessageTransfer(x)) => Ok(x),
            x => Err(anyhow!("Unexpected F1ap message {:?}", x)),
        }?;

        let pdcp_pdu = PdcpPdu(dl_rrc_message_transfer.rrc_container.0);

        let rrc_message_bytes = pdcp_pdu.view_inner()?;

        let rrc_setup = match DlCcchMessage::from_bytes(rrc_message_bytes)?.message {
            DlCcchMessageType::C1(C1_1::RrcSetup(x)) => Ok(x),
            x => Err(anyhow!("Unexpected RRC message {:?}", x)),
        }?;

        // Build RRC Setup Response
        let rrc_setup_complete = UlDcchMessage {
            message: UlDcchMessageType::C1(C1_6::RrcSetupComplete(RrcSetupComplete {
                rrc_transaction_identifier: rrc_setup.rrc_transaction_identifier,
                critical_extensions: CriticalExtensions22::RrcSetupComplete(RrcSetupCompleteIEs {
                    selected_plmn_identity: 1,
                    registered_amf: None,
                    guami_type: None,
                    s_nssai_list: None,
                    dedicated_nas_message: DedicatedNasMessage(nas_message),
                    ng_5g_s_tmsi_value: None,
                    late_non_critical_extension: None,
                    non_critical_extension: None,
                }),
            })),
        };

        info!(
            &logger,
            "DU sends UlRrcMessageTransfer containing RrcSetupComplete containing NAS Registration Request"
        );
        self.send_ul_rrc(rrc_setup_complete, logger).await
    }

    pub async fn send_nas(&self, nas_bytes: Vec<u8>, logger: &Logger) -> Result<()> {
        let rrc = UlDcchMessage {
            message: UlDcchMessageType::C1(C1_6::UlInformationTransfer(UlInformationTransfer {
                critical_extensions: CriticalExtensions37::UlInformationTransfer(
                    UlInformationTransferIEs {
                        dedicated_nas_message: Some(DedicatedNasMessage(nas_bytes)),
                        late_non_critical_extension: None,
                    },
                ),
            })),
        };
        self.send_ul_rrc(rrc, &logger).await
    }

    async fn send_ul_rrc(&self, rrc: UlDcchMessage, logger: &Logger) -> Result<()> {
        // Encapsulate RRC message in PDCP PDU.
        let rrc_bytes = rrc.into_bytes()?;
        let pdcp_pdu = PdcpPdu::encode(&rrc_bytes);

        // Wrap it in an UL Rrc Message Transfer
        let f1_indication = UlRrcMessageTransferProcedure::encode_request(UlRrcMessageTransfer {
            gnb_cu_ue_f1ap_id: GnbCuUeF1apId(1),
            gnb_du_ue_f1ap_id: GnbDuUeF1apId(1),
            srb_id: SrbId(1),
            rrc_container: RrcContainer(pdcp_pdu.bytes()),
            selected_plmn_id: None,
            new_gnb_du_ue_f1ap_id: None,
        })?;

        self.sender.send_message(f1_indication, logger).await
    }

    pub async fn receive_nas(&self) -> Result<Vec<u8>> {
        let dl_rrc_message_transfer = self.receive_dl_rrc().await?;
        nas_from_dl_transfer_rrc_container(dl_rrc_message_transfer.rrc_container)
    }

    pub async fn receive_dl_rrc(&self) -> Result<DlRrcMessageTransfer> {
        match self.recv().await {
            F1apPdu::InitiatingMessage(InitiatingMessage::DlRrcMessageTransfer(x)) => Ok(x),
            x => Err(anyhow!("Unexpected F1ap message {:?}", x)),
        }
    }

    pub async fn receive_ue_context_setup_request(
        &self,
        _logger: &Logger,
    ) -> Result<SecurityModeCommand> {
        let ue_context_setup_request = match self.recv().await {
            F1apPdu::InitiatingMessage(InitiatingMessage::UeContextSetupRequest(x)) => Ok(x),
            x => Err(anyhow!("Unexpected F1ap message {:?}", x)),
        }?;

        match match ue_context_setup_request.rrc_container {
            Some(x) => rrc_from_container(x)?,
            None => return Err(anyhow!("Expected Rrc container on UeContextSetupRequest",)),
        }
        .message
        {
            DlDcchMessageType::C1(C1_2::SecurityModeCommand(x)) => Ok(x),
            x => Err(anyhow!("Expected security mode command - got {:?}", x)),
        }
    }

    pub async fn send_ue_context_setup_response(&self, logger: &Logger) -> Result<()> {
        let cell_group_config =
            f1ap::CellGroupConfig(make_rrc_cell_group_config().into_bytes().unwrap());
        let ue_context_setup_response = F1apPdu::SuccessfulOutcome(
            SuccessfulOutcome::UeContextSetupResponse(UeContextSetupResponse {
                gnb_cu_ue_f1ap_id: GnbCuUeF1apId(1),
                gnb_du_ue_f1ap_id: GnbDuUeF1apId(1),
                du_to_cu_rrc_information: DuToCuRrcInformation {
                    cell_group_config,
                    meas_gap_config: None,
                    requested_p_max_fr1: None,
                },
                c_rnti: None,
                resource_coordination_transfer_container: None,
                full_configuration: None,
                drbs_setup_list: None,
                srbs_failed_to_be_setup_list: None,
                drbs_failed_to_be_setup_list: None,
                s_cell_failedto_setup_list: None,
                inactivity_monitoring_response: None,
                criticality_diagnostics: None,
                srbs_setup_list: None,
                bh_channels_setup_list: None,
                bh_channels_failed_to_be_setup_list: None,
                sl_drbs_setup_list: None,
                sl_drbs_failed_to_be_setup_list: None,
                requested_target_cell_global_id: None,
            }),
        )
        .into_bytes()?;
        self.sender
            .send_message(ue_context_setup_response, logger)
            .await
    }

    pub async fn send_security_mode_complete(
        &self,
        security_mode_command: &SecurityModeCommand,
        logger: &Logger,
    ) -> Result<()> {
        let security_mode_complete = UlDcchMessage {
            message: UlDcchMessageType::C1(C1_6::SecurityModeComplete(SecurityModeComplete {
                rrc_transaction_identifier: security_mode_command
                    .rrc_transaction_identifier
                    .clone(),
                critical_extensions: CriticalExtensions27::SecurityModeComplete(
                    SecurityModeCompleteIEs {
                        late_non_critical_extension: None,
                    },
                ),
            })),
        };
        self.send_ul_rrc(security_mode_complete, logger).await
    }

    pub async fn receive_rrc_reconfiguration(&self, _logger: &Logger) -> Result<Vec<u8>> {
        let dl_rrc_message_transfer = self.receive_dl_rrc().await?;
        let mut nas_messages =
            match rrc_from_container(dl_rrc_message_transfer.rrc_container)?.message {
                DlDcchMessageType::C1(C1_2::RrcReconfiguration(RrcReconfiguration {
                    critical_extensions:
                        CriticalExtensions15::RrcReconfiguration(RrcReconfigurationIEs {
                            non_critical_extension:
                                Some(RrcReconfigurationV1530IEs {
                                    dedicated_nas_message_list: Some(x),
                                    ..
                                }),
                            ..
                        }),
                    ..
                })) => Ok(x),
                _ => Err(anyhow!(
                    "Couldn't find NAS message list in Rrc Reconfiguration"
                )),
            }?;

        if nas_messages.len() != 1 {
            return Err(anyhow!("Expected a single NAS message in list"));
        };
        Ok(nas_messages.remove(0).0)
    }

    pub async fn send_rrc_reconfiguration_complete(&self, logger: &Logger) -> Result<()> {
        let security_mode_complete = UlDcchMessage {
            message: UlDcchMessageType::C1(C1_6::RrcReconfigurationComplete(
                RrcReconfigurationComplete {
                    rrc_transaction_identifier: RrcTransactionIdentifier(1),
                    critical_extensions: CriticalExtensions16::RrcReconfigurationComplete(
                        RrcReconfigurationCompleteIEs {
                            late_non_critical_extension: None,
                            non_critical_extension: None,
                        },
                    ),
                },
            )),
        };
        self.send_ul_rrc(security_mode_complete, logger).await
    }
}

fn make_rrc_cell_group_config() -> rrc::CellGroupConfig {
    rrc::CellGroupConfig {
        cell_group_id: CellGroupId(0),
        rlc_bearer_to_add_mod_list: None,
        rlc_bearer_to_release_list: None,
        mac_cell_group_config: None,
        physical_cell_group_config: None,
        sp_cell_config: None,
        s_cell_to_add_mod_list: None,
        s_cell_to_release_list: None,
    }
}

fn make_du_to_cu_rrc_container() -> DuToCuRrcContainer {
    // We also need a CellGroupConfig to give to the CU.
    let cell_group_config_ie = make_rrc_cell_group_config().into_bytes().unwrap();
    DuToCuRrcContainer(cell_group_config_ie)
}

fn rrc_from_container(rrc_container: RrcContainer) -> Result<DlDcchMessage> {
    let pdcp_pdu = PdcpPdu(rrc_container.0);
    let rrc_message_bytes = pdcp_pdu.view_inner()?;
    let m = DlDcchMessage::from_bytes(rrc_message_bytes)?;
    Ok(m)
}

fn nas_from_dl_transfer_rrc_container(rrc_container: RrcContainer) -> Result<Vec<u8>> {
    match rrc_from_container(rrc_container)?.message {
        DlDcchMessageType::C1(C1_2::DlInformationTransfer(DlInformationTransfer {
            critical_extensions:
                CriticalExtensions4::DlInformationTransfer(DlInformationTransferIEs {
                    dedicated_nas_message: Some(x),
                    ..
                }),
            ..
        })) => Ok(x.0),
        x => Err(anyhow!("Unexpected RRC message {:?}", x)),
    }
}
#[async_trait]
impl TnlaEventHandler for MockDu {
    async fn handle_event(&self, _event: TnlaEvent, _tnla_id: u32, _logger: &Logger) {
        self.internal_sender.send(None).await.unwrap();
    }

    async fn handle_message(
        &self,
        message: Message,
        _tnla_id: u32,
        _logger: &Logger,
    ) -> Option<Message> {
        self.internal_sender
            .send(Some(F1apPdu::from_bytes(&message).unwrap()))
            .await
            .unwrap();
        None
    }
}
