use also_net::{
    JsonCodec, SctpTransportProvider, ServerTransportProvider, TnlaEvent, TnlaEventHandler,
};
use async_channel::{Receiver, Sender};
use async_std::task::JoinHandle;
use async_trait::async_trait;
use common::ngap::NgapPdu;
use slog::{o, trace, Logger};
use std::fmt::Debug;
use stop_token::StopSource;

const NGAP_SCTP_PPID: u32 = 60;

pub struct MockAmf {
    pub stop_source: StopSource,
    pub receiver: Receiver<Option<NgapPdu>>,
    pub sender: SctpTransportProvider<JsonCodec<NgapPdu>, NgapPdu>,
    pub task: JoinHandle<()>,
    logger: Logger,
}

#[derive(Debug, Clone)]
struct Handler(pub Sender<Option<NgapPdu>>);

impl MockAmf {
    pub async fn new(amf_address: &str, logger: &Logger) -> MockAmf {
        let (internal_sender, receiver) = async_channel::unbounded();
        let logger = logger.new(o!("amf" => 1));
        let stop_source = StopSource::new();
        let server = SctpTransportProvider::new(NGAP_SCTP_PPID, JsonCodec::new());
        let task = server
            .clone()
            .serve(
                amf_address.to_string(),
                stop_source.token(),
                Handler(internal_sender),
                logger.clone(),
            )
            .await
            .expect("Server bind failed");

        MockAmf {
            receiver,
            stop_source,
            sender: server,
            task,
            logger,
        }
    }

    pub async fn expect_connection(&self) {
        // Wait for connection to be established - the mock TNLA event handler sends us an empty message to indicate this.
        trace!(self.logger, "Wait for connection from worker");
        assert!(self
            .receiver
            .recv()
            .await
            .expect("Failed mock recv")
            .is_none());
    }

    pub async fn receive_ngap_pdu(&self) -> NgapPdu {
        self.receiver
            .recv()
            .await
            .expect("Expected message")
            .expect("Expected message")
    }
}

#[async_trait]
impl TnlaEventHandler for Handler {
    type MessageType = NgapPdu;

    async fn handle_event(&self, _event: TnlaEvent, _tnla_id: u32, _logger: &Logger) {
        self.0.send(None).await.unwrap();
    }

    // TODO indicate whether it is UE or non UE associated?
    async fn handle_message(&self, message: NgapPdu, _tnla_id: u32, logger: &Logger) {
        trace!(logger, "Got message from GNB");
        self.0.send(Some(message)).await.unwrap();
    }
}
