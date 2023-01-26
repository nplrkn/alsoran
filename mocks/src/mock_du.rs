//! mock_du - enables a test script to assume the role of the GNB-DU on the F1 reference point

use crate::mock::{Mock, Pdu, ReceivedPdu};
use anyhow::{anyhow, bail, ensure, Result};
use bitvec::prelude::*;
use f1ap::*;
use net::{AperSerde, Binding, Indication, TransportProvider};
use pdcp::PdcpPdu;
use rrc::*;
use slog::{debug, info, o, Logger};
use std::ops::{Deref, DerefMut};

const F1AP_SCTP_PPID: u32 = 62;
const F1AP_BIND_PORT: u16 = 38472;

impl Pdu for F1apPdu {}

pub struct MockDu {
    mock: Mock<F1apPdu>,
}

pub struct UeContext {
    ue_id: u32,
    gnb_cu_ue_f1ap_id: Option<GnbCuUeF1apId>,
    pub binding: Binding,
}

impl Deref for MockDu {
    type Target = Mock<F1apPdu>;

    fn deref(&self) -> &Self::Target {
        &self.mock
    }
}

impl DerefMut for MockDu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mock
    }
}

impl MockDu {
    pub async fn new(logger: &Logger) -> MockDu {
        let logger = logger.new(o!("du" => 1));
        let mock = Mock::new(logger).await;
        MockDu { mock }
    }

    pub async fn terminate(self) {
        self.mock.terminate().await
    }

    pub async fn new_ue_context(&self, ue_id: u32, worker_ip: &str) -> Result<UeContext> {
        Ok(UeContext {
            ue_id,
            binding: self.transport.new_ue_binding_from_ip(worker_ip).await?,
            gnb_cu_ue_f1ap_id: None,
        })
    }

    pub async fn perform_f1_setup(&mut self, worker_ip: &String) -> Result<()> {
        let transport_address = format!("{}:{}", worker_ip, F1AP_BIND_PORT);
        info!(self.logger, "Connect to CU {}", transport_address);
        self.connect(&transport_address, "0.0.0.0", F1AP_SCTP_PPID)
            .await;
        self.send_f1_setup_request().await?;
        self.receive_f1_setup_response().await
    }

    async fn send_f1_setup_request(&self) -> Result<()> {
        let pdu =
            f1ap::F1apPdu::InitiatingMessage(InitiatingMessage::F1SetupRequest(F1SetupRequest {
                transaction_id: TransactionId(0),
                gnb_du_id: GnbDuId(123),
                gnb_du_rrc_version: RrcVersion {
                    latest_rrc_version: bitvec![u8, Msb0;0, 0, 0],
                    latest_rrc_version_enhanced: None,
                },
                gnb_du_name: None,
                gnb_du_served_cells_list: None,
                transport_layer_address_info: None,
                bap_address: None,
                extended_gnb_cu_name: None,
            }));
        info!(self.logger, "F1SetupRequest >>");
        self.send(pdu.into_bytes()?, None).await;
        Ok(())
    }

    async fn receive_f1_setup_response(&self) -> Result<()> {
        let pdu = self.receive_pdu().await;
        let F1apPdu::SuccessfulOutcome(SuccessfulOutcome::F1SetupResponse(_)) = pdu
        else {
            bail!("Unexpected F1ap message {:?}", pdu)
        };
        info!(self.logger, "F1SetupResponse <<");
        Ok(())
    }

    pub async fn perform_rrc_setup(
        &self,
        ue_context: &mut UeContext,
        nas_message: Vec<u8>,
    ) -> Result<()> {
        self.send_rrc_setup_request(ue_context).await.unwrap();
        let rrc_setup = self.receive_rrc_setup(ue_context).await.unwrap();
        self.send_rrc_setup_complete(ue_context, rrc_setup, nas_message)
            .await?;
        Ok(())
    }

    async fn send_rrc_setup_request(&self, ue_context: &UeContext) -> Result<()> {
        let logger = &self.logger;

        // Build RRC Setup Request
        let rrc_setup_request = UlCcchMessage {
            message: UlCcchMessageType::C1(C1_4::RrcSetupRequest(RrcSetupRequest {
                rrc_setup_request: RrcSetupRequestIEs {
                    ue_identity: InitialUeIdentity::Ng5gSTmsiPart1(bitvec![u8, Msb0; 0;39]),
                    establishment_cause: EstablishmentCause::MtAccess,
                    spare: bitvec![u8, Msb0;0;1],
                },
            })),
        }
        .into_bytes()?;

        let du_to_cu_rrc_container = Some(make_du_to_cu_rrc_container());

        // Wrap them in an F1 Initial UL Rrc Message Transfer.
        let f1_indication =
            InitialUlRrcMessageTransferProcedure::encode_request(InitialUlRrcMessageTransfer {
                gnb_du_ue_f1ap_id: GnbDuUeF1apId(ue_context.ue_id),
                nr_cgi: NrCgi {
                    plmn_identity: PlmnIdentity(vec![0, 1, 2]),
                    nr_cell_identity: NrCellIdentity(bitvec![u8,Msb0;0;36]),
                },
                c_rnti: CRnti(14),
                rrc_container: RrcContainer(rrc_setup_request),
                du_to_cu_rrc_container,
                sul_access_indication: None,
                transaction_id: TransactionId(1),
                ran_ue_id: None,
                rrc_container_rrc_setup_complete: None,
            })?;

        info!(logger, "InitialUlRrcMessageTransfer(RrcSetupRequest) >>");
        self.send(f1_indication, Some(ue_context.binding.assoc_id))
            .await;

        Ok(())
    }

    async fn receive_rrc_setup(&self, ue_context: &mut UeContext) -> Result<RrcSetup> {
        // Receive DL Rrc Message Transfer and extract RRC Setup
        let pdu = self.receive_pdu().await;
        let F1apPdu::InitiatingMessage(InitiatingMessage::DlRrcMessageTransfer(dl_rrc_message_transfer)) = pdu
        else {
            bail!("Unexpected F1ap message {:?}", pdu)
        };

        // A Rrc Setup flows as a DlCcchMessage on SRB0.  Check this is indeed for SRB0.
        assert_eq!(dl_rrc_message_transfer.srb_id.0, 0);

        ue_context.gnb_cu_ue_f1ap_id = Some(dl_rrc_message_transfer.gnb_cu_ue_f1ap_id);
        let pdcp_pdu = PdcpPdu(dl_rrc_message_transfer.rrc_container.0);
        let rrc_message_bytes = pdcp_pdu.view_inner()?;

        // TODO - how to verify that this is indeed a DlCcchMessage rather than a DlDcchMessage.
        let message = DlCcchMessage::from_bytes(rrc_message_bytes)
            .unwrap()
            .message;

        let DlCcchMessageType::C1(C1_1::RrcSetup(rrc_setup)) = message else {
            bail!("Unexpected RRC message {:?}", message)
        };
        info!(&self.logger, "DlRrcMessageTransfer(RrcSetup) <<");
        Ok(rrc_setup)
    }

    async fn send_rrc_setup_complete(
        &self,
        ue_context: &UeContext,
        rrc_setup: RrcSetup,
        nas_message: Vec<u8>,
    ) -> Result<()> {
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
            &self.logger,
            "UlRrcMessageTransfer(RrcSetupComplete(NAS Registration Request)) >>"
        );
        self.send_ul_rrc(ue_context, rrc_setup_complete).await
    }

    pub async fn send_nas(&self, ue_context: &UeContext, nas_bytes: Vec<u8>) -> Result<()> {
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
        info!(
            &self.logger,
            "UlRrcMessageTransfer(UlInformationTransfer(Nas)) >>"
        );
        self.send_ul_rrc(ue_context, rrc).await
    }

    async fn send_ul_rrc(&self, ue_context: &UeContext, rrc: UlDcchMessage) -> Result<()> {
        let gnb_cu_ue_f1ap_id = ue_context.gnb_cu_ue_f1ap_id.unwrap();

        // Encapsulate RRC message in PDCP PDU.
        let rrc_bytes = rrc.into_bytes()?;
        let pdcp_pdu = PdcpPdu::encode(&rrc_bytes);

        // Wrap it in an UL Rrc Message Transfer
        let f1_indication = UlRrcMessageTransferProcedure::encode_request(UlRrcMessageTransfer {
            gnb_cu_ue_f1ap_id,
            gnb_du_ue_f1ap_id: GnbDuUeF1apId(ue_context.ue_id),
            srb_id: SrbId(1),
            rrc_container: RrcContainer(pdcp_pdu.into()),
            selected_plmn_id: None,
            new_gnb_du_ue_f1ap_id: None,
        })?;

        self.send(f1_indication, Some(ue_context.binding.assoc_id))
            .await;
        Ok(())
    }

    pub async fn receive_nas(&self, ue_context: &UeContext) -> Result<Vec<u8>> {
        let dl_rrc_message_transfer = self.receive_dl_rrc(ue_context).await?;
        info!(
            &self.logger,
            "DlRrcMessageTransfer(DlInformationTransfer(Nas)) <<"
        );
        nas_from_dl_transfer_rrc_container(dl_rrc_message_transfer.rrc_container)
    }

    async fn receive_dl_rrc(&self, ue_context: &UeContext) -> Result<DlRrcMessageTransfer> {
        let ReceivedPdu { pdu, assoc_id } = self.receive_pdu_with_assoc_id().await;

        // Check that the PDU arrived on the expected binding.
        assert_eq!(assoc_id, ue_context.binding.assoc_id);

        let F1apPdu::InitiatingMessage(InitiatingMessage::DlRrcMessageTransfer(dl_rrc_message_transfer)) = pdu
        else {
            bail!("Unexpected F1ap message {:?}", pdu)
        };

        assert_eq!(
            dl_rrc_message_transfer.gnb_du_ue_f1ap_id.0,
            ue_context.ue_id
        );
        Ok(dl_rrc_message_transfer)
    }

    pub async fn receive_security_mode_command(
        &self,
        ue_context: &UeContext,
    ) -> Result<SecurityModeCommand> {
        let dl_rrc_message_transfer = self.receive_dl_rrc(ue_context).await?;

        // A Rrc Setup flows as a DlDcchMessage on SRB1.  Check this is indeed for SRB1.
        assert_eq!(dl_rrc_message_transfer.srb_id.0, 1);

        let message = rrc_from_container(dl_rrc_message_transfer.rrc_container)?.message;
        let DlDcchMessageType::C1(C1_2::SecurityModeCommand(security_mode_command)) = message else {
            bail!("Expected security mode command - got {:?}", message)
        };
        info!(&self.logger, "DlRrcMessageTransfer(SecurityModeCommand) <<");
        Ok(security_mode_command)
    }

    pub async fn handle_ue_context_setup(&self, ue_context: &UeContext) -> Result<()> {
        let ReceivedPdu { pdu, assoc_id } = self.receive_pdu_with_assoc_id().await;
        let _ = self.check_ue_context_setup_request(pdu, ue_context)?;
        info!(&self.logger, "UeContextSetupRequest <<");

        // Code to check for an Rrc Container of a particular type
        // match match rrc_container {
        //     Some(x) => rrc_from_container(x)?,
        //     None => return Err(anyhow!("Expected Rrc container on UeContextSetupRequest",)),
        // }
        // .message
        // {
        //     DlDcchMessageType::C1(C1_2::SecurityModeCommand(x)) => {
        //         info!(
        //             &self.logger,
        //             "UeContextSetupRequest(SecurityModeCommand) <<"
        //         );
        //         Ok(x)
        //     }
        //     x => Err(anyhow!("Expected security mode command - got {:?}", x)),
        // }

        let ue_context_setup_response = self.build_ue_context_setup_response(ue_context);
        info!(&self.logger, "UeContextSetupResponse >>");
        self.send(ue_context_setup_response.into_bytes()?, Some(assoc_id))
            .await;

        Ok(())
    }

    pub fn check_ue_context_setup_request(
        &self,
        pdu: F1apPdu,
        ue_context: &UeContext,
    ) -> Result<Option<RrcContainer>> {
        let F1apPdu::InitiatingMessage(InitiatingMessage::UeContextSetupRequest(ue_context_setup_request)) = pdu
        else {
            bail!("Unexpected F1ap message {:?}", pdu)
        };

        ensure!(
            matches!(ue_context_setup_request.gnb_du_ue_f1ap_id, Some(GnbDuUeF1apId(x)) if x == ue_context.ue_id),
            "Bad Ue Id"
        );

        Ok(ue_context_setup_request.rrc_container)
    }

    pub fn build_ue_context_setup_response(&self, ue_context: &UeContext) -> F1apPdu {
        let gnb_cu_ue_f1ap_id = ue_context.gnb_cu_ue_f1ap_id.unwrap();
        let cell_group_config =
            f1ap::CellGroupConfig(make_rrc_cell_group_config().into_bytes().unwrap());
        F1apPdu::SuccessfulOutcome(SuccessfulOutcome::UeContextSetupResponse(
            UeContextSetupResponse {
                gnb_cu_ue_f1ap_id,
                gnb_du_ue_f1ap_id: GnbDuUeF1apId(ue_context.ue_id),
                du_to_cu_rrc_information: DuToCuRrcInformation {
                    cell_group_config,
                    meas_gap_config: None,
                    requested_p_max_fr1: None,
                    drx_long_cycle_start_offset: None,
                    selected_band_combination_index: None,
                    selected_feature_set_entry_index: None,
                    ph_info_scg: None,
                    requested_band_combination_index: None,
                    requested_feature_set_entry_index: None,
                    drx_config: None,
                    pdcch_blind_detection_scg: None,
                    requested_pdcch_blind_detection_scg: None,
                    ph_info_mcg: None,
                    meas_gap_sharing_config: None,
                    sl_phy_mac_rlc_config: None,
                    sl_config_dedicated_eutra_info: None,
                    requested_p_max_fr2: None,
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
            },
        ))
    }

    pub async fn send_security_mode_complete(
        &self,
        ue_context: &UeContext,
        security_mode_command: &SecurityModeCommand,
    ) -> Result<()> {
        let security_mode_complete = UlDcchMessage {
            message: UlDcchMessageType::C1(C1_6::SecurityModeComplete(SecurityModeComplete {
                rrc_transaction_identifier: security_mode_command.rrc_transaction_identifier,
                critical_extensions: CriticalExtensions27::SecurityModeComplete(
                    SecurityModeCompleteIEs {
                        late_non_critical_extension: None,
                    },
                ),
            })),
        };
        info!(
            &self.logger,
            "UlRrcMessageTransfer(SecurityModeComplete) >>"
        );
        self.send_ul_rrc(ue_context, security_mode_complete).await
    }

    pub async fn receive_rrc_reconfiguration(&self, ue_context: &UeContext) -> Result<Vec<u8>> {
        let dl_rrc_message_transfer = self.receive_dl_rrc(ue_context).await?;
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
                })) => {
                    info!(
                        &self.logger,
                        "DlRrcMessageTransfer(RrcReconfiguration(Nas)) <<"
                    );
                    Ok(x)
                }
                _ => Err(anyhow!(
                    "Couldn't find NAS message list in Rrc Reconfiguration"
                )),
            }?;

        ensure!(
            nas_messages.len() == 1,
            "Expected a single NAS message in list"
        );
        Ok(nas_messages.remove(0).0)
    }

    pub async fn send_rrc_reconfiguration_complete(&self, ue_context: &UeContext) -> Result<()> {
        let rrc_reconfiguration_complete = UlDcchMessage {
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
        info!(
            &self.logger,
            "UlRrcMessageTransfer(RrcReconfigurationComplete) >>"
        );
        self.send_ul_rrc(ue_context, rrc_reconfiguration_complete)
            .await
    }

    pub async fn handle_cu_configuration_update(
        &mut self,
        expected_addr_string: &String,
    ) -> Result<()> {
        let expected_address =
            TransportLayerAddress(net::ip_bits_from_string(expected_addr_string)?);
        let (transaction_id, assoc_id) = self
            .receive_gnb_cu_configuration_update(&expected_address)
            .await?;
        let transport_address = format!("{}:{}", expected_addr_string, F1AP_BIND_PORT);
        info!(self.logger, "Connect to CU {}", transport_address);
        self.connect(&transport_address, "0.0.0.0", F1AP_SCTP_PPID)
            .await;
        self.send_gnb_cu_configuration_update_acknowledge(
            transaction_id,
            expected_address,
            assoc_id,
        )
        .await
    }

    async fn receive_gnb_cu_configuration_update(
        &self,
        expected_address: &TransportLayerAddress,
    ) -> Result<(TransactionId, u32)> {
        debug!(self.logger, "Wait for Cu Configuration Update");
        let ReceivedPdu { pdu, assoc_id } = self.receive_pdu_with_assoc_id().await;

        let F1apPdu::InitiatingMessage(InitiatingMessage::GnbCuConfigurationUpdate(cu_configuration_update)) = pdu
        else {
            bail!("Expected GnbCuConfigurationUpdate, got {:?}", pdu)
        };
        info!(self.logger, "GnbCuConfigurationUpdate <<");

        let gnb_cu_tnl_association_to_add_list = cu_configuration_update
            .gnb_cu_tnl_association_to_add_list
            .expect("Expected gnb_cu_cp_tnla_to_add_list to be present");
        match &gnb_cu_tnl_association_to_add_list
            .0
            .first()
            .expect("Expected nonempty gnb_cu_tnl_association_to_add_list")
            .tnl_association_transport_layer_address
        {
            CpTransportLayerAddress::EndpointIpAddress(ref x) => {
                assert_eq!(x.0, expected_address.0);
            }
            CpTransportLayerAddress::EndpointIpAddressAndPort(_) => {
                panic!("Alsoran CU-CP doesn't specify a port")
            }
        };

        Ok((cu_configuration_update.transaction_id, assoc_id))
    }

    async fn send_gnb_cu_configuration_update_acknowledge(
        &self,
        transaction_id: TransactionId,
        transport_layer_address: TransportLayerAddress,
        assoc_id: u32,
    ) -> Result<()> {
        let pdu = f1ap::F1apPdu::SuccessfulOutcome(
            SuccessfulOutcome::GnbCuConfigurationUpdateAcknowledge(
                GnbCuConfigurationUpdateAcknowledge {
                    transaction_id,
                    cells_failed_to_be_activated_list: None,
                    criticality_diagnostics: None,
                    gnb_cu_tnl_association_setup_list: Some(GnbCuTnlAssociationSetupList(vec![
                        GnbCuTnlAssociationSetupItem {
                            tnl_association_transport_layer_address:
                                CpTransportLayerAddress::EndpointIpAddress(transport_layer_address),
                        },
                    ])),
                    gnb_cu_tnl_association_failed_to_setup_list: None,
                    dedicated_si_delivery_needed_ue_list: None,
                    transport_layer_address_info: None,
                },
            ),
        );

        info!(self.logger, "GnbCuConfigurationUpdateAcknowledge >>");
        self.send(pdu.into_bytes()?, Some(assoc_id)).await;
        Ok(())
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
