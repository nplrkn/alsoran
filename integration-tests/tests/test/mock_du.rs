use anyhow::{anyhow, Result};
use async_channel::{Receiver, Sender};
use async_trait::async_trait;
use bitvec::prelude::*;
use f1ap::*;
use net::{AperSerde, Message, TransportProvider};
use net::{SctpTransportProvider, TnlaEvent, TnlaEventHandler};
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

        match self.receiver.recv().await? {
            Some(_response) => {
                info!(self.logger, "Got response from CU");
                Ok(())
            }
            None => Err(anyhow!(
                "Unexpected empty channel waiting for F1 Setup response"
            )),
        }?;

        Ok(())
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
