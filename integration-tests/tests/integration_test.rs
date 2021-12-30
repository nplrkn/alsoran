use also_net::{
    JsonCodec, SctpTransportProvider, ServerTransportProvider, TnlaEvent, TnlaEventHandler,
    TransportProvider,
};
use async_channel::{Receiver, Sender};
use async_std;
use bitvec::vec::BitVec;
use common::ngap::*;
use slog::{info, o, trace, Logger};
use std::{panic, process};
use stop_token::StopSource;

#[derive(Debug, Clone)]
struct MockAmf {
    sender: Sender<Option<NgapPdu>>,
}

impl MockAmf {
    pub fn new() -> (MockAmf, Receiver<Option<NgapPdu>>) {
        let (sender, their_receiver) = async_channel::unbounded();

        (MockAmf { sender }, their_receiver)
    }
}

#[async_trait::async_trait]
impl TnlaEventHandler for MockAmf {
    type MessageType = NgapPdu;

    async fn handle_event(&self, _event: TnlaEvent, _tnla_id: u32, _logger: &Logger) {
        self.sender.send(None).await.unwrap();
    }

    // TODO indicate whether it is UE or non UE associated?
    async fn handle_message(&self, message: NgapPdu, _tnla_id: u32, logger: &Logger) {
        trace!(logger, "Got message from GNB");
        self.sender.send(Some(message)).await.unwrap();
    }
}

const NGAP_SCTP_PPID: u32 = 60;

#[async_std::test]
async fn run_everything() {
    let logger = common::logging::test_init();

    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        orig_hook(panic_info);
        process::exit(1);
    }));

    // Listen on the AMF SCTP port so that when the worker starts up it will be able to connect.
    let amf_address = "127.0.0.1:38212";
    let server_stop_source = StopSource::new();
    let server_stop_token = server_stop_source.token();

    // We use a JSON encoding for now given that we do not have a working ASN.1 Per codec
    let server = SctpTransportProvider::new(NGAP_SCTP_PPID, JsonCodec::new());
    let (amf_handler, amf_receiver) = MockAmf::new();
    let server_task = server
        .clone()
        .serve(
            amf_address.to_string(),
            server_stop_token,
            amf_handler,
            logger.new(o!("nodetype"=> "mock amf")),
        )
        .await
        .expect("Server bind failed");

    let (coord_stop_source, coord_task) = coordinator::spawn(logger.new(o!("nodetype"=> "cu-c")));
    let (worker_stop_source, worker_task) = worker::spawn(
        logger.new(o!("nodetype"=> "cu-w")),
        JsonCodec::new(),
        JsonCodec::new(),
    );

    // Wait for connection to be established - the mock sends us an empty message to indicate this.
    assert!(amf_receiver
        .recv()
        .await
        .expect("Failed mock recv")
        .is_none());

    // Catch NG Setup from the GNB
    info!(logger, "Wait for NG Setup from GNB");

    // TODO - hide away these expect calls
    let pdu: NgapPdu = amf_receiver
        .recv()
        .await
        .expect("Expected message")
        .expect("Expected message");
    if let NgapPdu::InitiatingMessage(InitiatingMessage {
        value: InitiatingMessageValue::IdNgSetup(_ng_setup),
        ..
    }) = pdu
    {
        info!(logger, "Got NG Setup, send setup response");
    } else {
        panic!("Not an NG setup");
    }

    // TODO - due to an apparent bug in the decoder, this has a spurious 00 on the end.
    // TODO - deduplicate with worker test
    let ng_setup_response = NgapPdu::InitiatingMessage(InitiatingMessage {
        procedure_code: ProcedureCode(21),
        criticality: Criticality(Criticality::REJECT),
        value: InitiatingMessageValue::IdNgSetup(NgSetupRequest {
            protocol_i_es: NgSetupRequestProtocolIEs(vec![NgSetupRequestProtocolIEsItem {
                id: ProtocolIeId(27),
                criticality: Criticality(Criticality::REJECT),
                value: NgSetupRequestProtocolIEsItemValue::IdGlobalRanNodeId(
                    GlobalRanNodeId::GlobalGnbId(GlobalGnbId {
                        plmn_identity: PlmnIdentity(vec![2, 3, 2, 1, 5, 6]),
                        gnb_id: GnbId::GnbId(BitString26(BitVec::from_element(0x10))),
                        ie_extensions: None,
                    }),
                ),
            }]),
        }),
    });

    server
        .send_pdu(ng_setup_response, &logger)
        .await
        .expect("Failed mock send");

    info!(logger, "Terminate coordinator");
    drop(coord_stop_source);

    info!(logger, "Terminate worker");
    drop(worker_stop_source);

    info!(logger, "Wait for worker to terminate connection");
    assert!(amf_receiver
        .recv()
        .await
        .expect("Expected connection termination")
        .is_none());

    info!(logger, "Terminate mock AMF");
    drop(server_stop_source);

    info!(logger, "Wait for all tasks to terminate cleanly");
    coord_task.await;
    worker_task.await;
    server_task.await;
    drop(logger);
}
