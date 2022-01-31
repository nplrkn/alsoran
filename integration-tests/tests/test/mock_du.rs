use also_net::{ClientTransportProvider, TransportProvider};
use also_net::{JsonCodec, SctpTransportProvider, TnlaEvent, TnlaEventHandler};
use anyhow::{anyhow, Result};
use async_channel::{Receiver, Sender};
use async_trait::async_trait;
use bitvec::prelude::*;
use f1ap::*;
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
    pub sender: SctpTransportProvider<JsonCodec<F1apPdu>, F1apPdu>,
    internal_sender: Sender<Option<F1apPdu>>,
    logger: Logger,
}

impl MockDu {
    pub async fn new(logger: &Logger) -> (MockDu, StopSource) {
        let logger = logger.new(o!("du" => 1));
        let (internal_sender, receiver) = async_channel::unbounded();
        let stop_source = StopSource::new();
        let sender = SctpTransportProvider::new(F1AP_SCTP_PPID, JsonCodec::new());

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
                self.clone(),
                self.stop_token.clone(),
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
        // From the ASN.1, TransactionID, GNB-DU-ID and RRC-Version are mandatory.
        // F1SetupRequestIEs F1AP-PROTOCOL-IES ::= {
        //     { ID id-TransactionID					CRITICALITY reject	TYPE TransactionID						PRESENCE mandatory	}|
        //     { ID id-gNB-DU-ID						CRITICALITY reject	TYPE GNB-DU-ID							PRESENCE mandatory	}|
        //     { ID id-gNB-DU-Name						CRITICALITY ignore	TYPE GNB-DU-Name						PRESENCE optional	}|
        //     { ID id-gNB-DU-Served-Cells-List		CRITICALITY reject	TYPE GNB-DU-Served-Cells-List			PRESENCE optional	}|
        //     { ID id-GNB-DU-RRC-Version				CRITICALITY reject	TYPE RRC-Version						PRESENCE mandatory	}|
        //     { ID id-Transport-Layer-Address-Info	CRITICALITY ignore	TYPE Transport-Layer-Address-Info		PRESENCE optional	}|
        //     { ID id-BAPAddress						CRITICALITY ignore	TYPE BAPAddress							PRESENCE optional	}|
        //     { ID id-Extended-GNB-CU-Name			CRITICALITY ignore	TYPE Extended-GNB-CU-Name				PRESENCE optional	},
        //     ...
        // }

        let pdu = f1ap::F1SetupRequest {
            transaction_id: TransactionId(0),
            gnb_du_id: GnbDuId(123),
            gnb_du_rrc_version: RrcVersion {
                latest_rrc_version: bitvec![Msb0, u8;0, 0, 0],
                latest_rrc_version_enhanced: Some([1, 2, 3]),
            },
        };

        // let pdu = F1apPdu::InitiatingMessage(InitiatingMessage {
        //     procedure_code: ProcedureCode(1),
        //     criticality: Criticality(Criticality::REJECT),
        //     value: InitiatingMessageValue::IdF1Setup(F1SetupRequest {
        //         protocol_i_es: F1SetupRequestProtocolIEs(vec![
        //             F1SetupRequestProtocolIEsEntry {
        //                 id: ProtocolIeId(78),
        //                 criticality: Criticality(Criticality::REJECT),
        //                 value: F1SetupRequestProtocolIEsEntryValue::IdTransactionId(TransactionId(
        //                     0,
        //                 )),
        //             },
        //             F1SetupRequestProtocolIEsEntry {
        //                 id: ProtocolIeId(42),
        //                 criticality: Criticality(Criticality::REJECT),
        //                 value: F1SetupRequestProtocolIEsEntryValue::IdGNbDuId(GnbDuId(1)),
        //             },
        //             F1SetupRequestProtocolIEsEntry {
        //                 id: ProtocolIeId(171),
        //                 criticality: Criticality(Criticality::REJECT),
        //                 value: F1SetupRequestProtocolIEsEntryValue::IdGnbDuRrcVersion(RrcVersion {
        //                     latest_rrc_version: BitString148(bitvec![Msb0,u8; 0, 0, 0]),
        //                     ie_extensions: None,
        //                 }),
        //             },
        //         ]),
        //     }),
        // });

        self.sender.send_pdu(pdu.into(), &self.logger).await?;
        info!(self.logger, "Wait for F1 Setup response from GNB");

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
impl TnlaEventHandler<F1apPdu> for MockDu {
    async fn handle_event(&self, _event: TnlaEvent, _tnla_id: u32, _logger: &Logger) {
        self.internal_sender.send(None).await.unwrap();
    }

    async fn handle_message(&self, message: F1apPdu, _tnla_id: u32, logger: &Logger) {
        trace!(logger, "Got message from CU");
        self.internal_sender.send(Some(message)).await.unwrap();
    }
}
