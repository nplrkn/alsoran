use anyhow::{anyhow, Result};
use async_channel::{Receiver, Sender};
use async_trait::async_trait;
use bitvec::prelude::*;
use f1ap::*;
use net::{AperSerde, Message, Procedure, TransportProvider};
use net::{SctpTransportProvider, TnlaEvent, TnlaEventHandler};
use rrc::*;
use slog::{info, o, trace, Logger};
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
        trace!(self.logger, "Wait for connection to be accepted by CU");
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

    pub async fn perform_rrc_setup(&self, logger: &Logger) -> Result<()> {
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

        // We also need a CellGroupConfig to give to the CU.
        let cell_group_config_ie = rrc::CellGroupConfig {
            cell_group_id: CellGroupId(0),
            rlc_bearer_to_add_mod_list: None,
            rlc_bearer_to_release_list: None,
            mac_cell_group_config: None,
            physical_cell_group_config: None,
            sp_cell_config: None,
            s_cell_to_add_mod_list: None,
            s_cell_to_release_list: None,
        }
        .into_bytes()?;

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
                du_to_cu_rrc_container: Some(DuToCuRrcContainer(cell_group_config_ie)),
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

        let message = self.recv().await;

        // Receive DL Rrc Message Transfer and extract RRC Setup
        let _rrc_setup = match message {
            F1apPdu::InitiatingMessage(InitiatingMessage::DlRrcMessageTransfer(_)) => {
                info!(logger, "Received Rrc Setup");
                Ok(())
            }
            m => Err(anyhow!("Unexpected message {:?}", m)),
        }?;

        // Build RRC Setup Response

        // Wrap it in an UL Rrc Message Transfer

        // Send
        unimplemented!()
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
        logger: &Logger,
    ) -> Option<Message> {
        trace!(logger, "Got message from CU");
        self.internal_sender
            .send(Some(F1apPdu::from_bytes(&message).unwrap()))
            .await
            .unwrap();
        None
    }
}
