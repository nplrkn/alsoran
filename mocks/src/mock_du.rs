//! mock_du - enables a test script to assume the role of the GNB-DU on the F1 reference point

use crate::mock::{Mock, Pdu};
use anyhow::{anyhow, Result};
use bitvec::prelude::*;
use f1ap::*;
use net::{AperSerde, Indication};
use pdcp::PdcpPdu;
use rrc::*;
use slog::{info, o, Logger};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

impl Pdu for F1apPdu {}

pub struct MockDu {
    mock: Mock<F1apPdu>,
    ues: HashMap<u32, UeContext>,
}

struct UeContext {
    gnb_cu_ue_f1ap_id: Option<GnbCuUeF1apId>,
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
        MockDu {
            mock,
            ues: HashMap::new(),
        }
    }

    pub async fn terminate(self) {
        self.mock.terminate().await
    }

    pub async fn perform_f1_setup(&self) -> Result<()> {
        self.send_f1_setup_request().await?;
        self.receive_f1_setup_response().await;
        Ok(())
    }

    async fn send_f1_setup_request(&self) -> Result<()> {
        let pdu =
            f1ap::F1apPdu::InitiatingMessage(InitiatingMessage::F1SetupRequest(F1SetupRequest {
                transaction_id: TransactionId(0),
                gnb_du_id: GnbDuId(123),
                gnb_du_rrc_version: RrcVersion {
                    latest_rrc_version: bitvec![u8, Msb0;0, 0, 0],
                },
                gnb_du_name: None,
                gnb_du_served_cells_list: None,
                transport_layer_address_info: None,
                bap_address: None,
                extended_gnb_cu_name: None,
            }));
        info!(self.logger, "F1SetupRequest >>");
        self.send(pdu.into_bytes()?).await;
        Ok(())
    }

    async fn receive_f1_setup_response(&self) {
        let _response = self.receive_pdu().await;
        info!(self.logger, "F1SetupResponse <<");
    }

    pub async fn perform_rrc_setup(&mut self, ue_id: u32, nas_message: Vec<u8>) -> Result<()> {
        self.ues.insert(
            ue_id,
            UeContext {
                gnb_cu_ue_f1ap_id: None,
            },
        );
        self.send_rrc_setup_request(ue_id).await?;
        let rrc_setup = self.receive_rrc_setup(ue_id).await?;
        self.send_rrc_setup_complete(ue_id, rrc_setup, nas_message)
            .await
    }

    async fn send_rrc_setup_request(&self, ue_id: u32) -> Result<()> {
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
                gnb_du_ue_f1ap_id: GnbDuUeF1apId(ue_id),
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

        self.send(f1_indication).await;
        Ok(())
    }

    async fn receive_rrc_setup(&mut self, ue_id: u32) -> Result<RrcSetup> {
        // Receive DL Rrc Message Transfer and extract RRC Setup
        let dl_rrc_message_transfer = match self.receive_pdu().await {
            F1apPdu::InitiatingMessage(InitiatingMessage::DlRrcMessageTransfer(x)) => Ok(x),
            x => Err(anyhow!("Unexpected F1ap message {:?}", x)),
        }?;
        self.ues.get_mut(&ue_id).unwrap().gnb_cu_ue_f1ap_id =
            Some(dl_rrc_message_transfer.gnb_cu_ue_f1ap_id);
        let pdcp_pdu = PdcpPdu(dl_rrc_message_transfer.rrc_container.0);
        let rrc_message_bytes = pdcp_pdu.view_inner()?;
        let rrc_setup = match DlCcchMessage::from_bytes(rrc_message_bytes)?.message {
            DlCcchMessageType::C1(C1_1::RrcSetup(x)) => Ok(x),
            x => Err(anyhow!("Unexpected RRC message {:?}", x)),
        }?;
        info!(&self.logger, "DlRrcMessageTransfer(RrcSetup) <<");
        Ok(rrc_setup)
    }

    async fn send_rrc_setup_complete(
        &self,
        ue_id: u32,
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
        self.send_ul_rrc(ue_id, rrc_setup_complete).await
    }

    pub async fn send_nas(&self, ue_id: u32, nas_bytes: Vec<u8>) -> Result<()> {
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
        self.send_ul_rrc(ue_id, rrc).await
    }

    async fn send_ul_rrc(&self, ue_id: u32, rrc: UlDcchMessage) -> Result<()> {
        let gnb_cu_ue_f1ap_id = self.ues[&ue_id].gnb_cu_ue_f1ap_id.clone().unwrap();

        // Encapsulate RRC message in PDCP PDU.
        let rrc_bytes = rrc.into_bytes()?;
        let pdcp_pdu = PdcpPdu::encode(&rrc_bytes);

        // Wrap it in an UL Rrc Message Transfer
        let f1_indication = UlRrcMessageTransferProcedure::encode_request(UlRrcMessageTransfer {
            gnb_cu_ue_f1ap_id,
            gnb_du_ue_f1ap_id: GnbDuUeF1apId(ue_id),
            srb_id: SrbId(1),
            rrc_container: RrcContainer(pdcp_pdu.into()),
            selected_plmn_id: None,
            new_gnb_du_ue_f1ap_id: None,
        })?;

        self.send(f1_indication).await;
        Ok(())
    }

    pub async fn receive_nas(&self, ue_id: u32) -> Result<Vec<u8>> {
        let dl_rrc_message_transfer = self.receive_dl_rrc(ue_id).await?;
        info!(
            &self.logger,
            "DlRrcMessageTransfer(DlInformationTransfer(Nas)) <<"
        );
        nas_from_dl_transfer_rrc_container(dl_rrc_message_transfer.rrc_container)
    }

    async fn receive_dl_rrc(&self, ue_id: u32) -> Result<DlRrcMessageTransfer> {
        let dl_rrc_message_transfer = match self.receive_pdu().await {
            F1apPdu::InitiatingMessage(InitiatingMessage::DlRrcMessageTransfer(x)) => Ok(x),
            x => Err(anyhow!("Unexpected F1ap message {:?}", x)),
        }?;

        assert_eq!(dl_rrc_message_transfer.gnb_du_ue_f1ap_id.0, ue_id);
        Ok(dl_rrc_message_transfer)
    }

    pub async fn receive_ue_context_setup_request(
        &self,
        ue_id: u32,
    ) -> Result<SecurityModeCommand> {
        let ue_context_setup_request = match self.receive_pdu().await {
            F1apPdu::InitiatingMessage(InitiatingMessage::UeContextSetupRequest(x)) => Ok(x),
            x => Err(anyhow!("Unexpected F1ap message {:?}", x)),
        }?;

        match ue_context_setup_request.gnb_du_ue_f1ap_id {
            Some(GnbDuUeF1apId(x)) if x == ue_id => (),
            _ => panic!("Bad ue id"),
        }

        match match ue_context_setup_request.rrc_container {
            Some(x) => rrc_from_container(x)?,
            None => return Err(anyhow!("Expected Rrc container on UeContextSetupRequest",)),
        }
        .message
        {
            DlDcchMessageType::C1(C1_2::SecurityModeCommand(x)) => {
                info!(
                    &self.logger,
                    "UeContextSetupRequest(SecurityModeCommand) <<"
                );
                Ok(x)
            }
            x => Err(anyhow!("Expected security mode command - got {:?}", x)),
        }
    }

    pub async fn send_ue_context_setup_response(&self, ue_id: u32) -> Result<()> {
        let gnb_cu_ue_f1ap_id = self.ues[&ue_id].gnb_cu_ue_f1ap_id.clone().unwrap();
        let cell_group_config =
            f1ap::CellGroupConfig(make_rrc_cell_group_config().into_bytes().unwrap());
        let ue_context_setup_response = F1apPdu::SuccessfulOutcome(
            SuccessfulOutcome::UeContextSetupResponse(UeContextSetupResponse {
                gnb_cu_ue_f1ap_id,
                gnb_du_ue_f1ap_id: GnbDuUeF1apId(ue_id),
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
        info!(&self.logger, "UeContextSetupResponse >>");

        self.send(ue_context_setup_response).await;
        Ok(())
    }

    pub async fn send_security_mode_complete(
        &self,
        ue_id: u32,
        security_mode_command: &SecurityModeCommand,
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
        info!(
            &self.logger,
            "UlRrcMessageTransfer(SecurityModeComplete) >>"
        );
        self.send_ul_rrc(ue_id, security_mode_complete).await
    }

    pub async fn receive_rrc_reconfiguration(&self, ue_id: u32) -> Result<Vec<u8>> {
        let dl_rrc_message_transfer = self.receive_dl_rrc(ue_id).await?;
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

        if nas_messages.len() != 1 {
            return Err(anyhow!("Expected a single NAS message in list"));
        };
        Ok(nas_messages.remove(0).0)
    }

    pub async fn send_rrc_reconfiguration_complete(&self, ue_id: u32) -> Result<()> {
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
        self.send_ul_rrc(ue_id, rrc_reconfiguration_complete).await
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
