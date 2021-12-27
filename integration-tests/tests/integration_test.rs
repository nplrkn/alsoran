use async_channel::{Receiver, Sender};
use async_std;
//use backtrace::Backtrace;
use common::sctp_server_transport_provider::SctpServerTransportProvider;
use common::transport_provider::{Handler, Message, ServerTransportProvider, TransportProvider};
use slog::{info, o, Logger};
//use std::panic;
//use std::process;
use common::ngap::*;
use serde_json;
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
impl Handler for MockAmf {
    async fn tnla_established(&self, _tnla_id: u32, _logger: &Logger) {
        self.sender.send(None).await.unwrap();
    }

    async fn tnla_terminated(&self, _tnla_id: u32, _logger: &Logger) {
        self.sender.send(None).await.unwrap();
    }

    async fn recv_non_ue_associated(&self, m: Message, _logger: &Logger) {
        let ngap_pdu: NgapPdu = serde_json::from_str(std::str::from_utf8(&m).unwrap()).unwrap();
        self.sender.send(Some(ngap_pdu)).await.unwrap();
    }
}

const NGAP_SCTP_PPID: u32 = 60;

#[async_std::test]
async fn run_everything() {
    let logger = common::logging::test_init();

    // let orig_hook = panic::take_hook();
    // let logger_clone = logger.clone();
    // panic::set_hook(Box::new(move |panic_info| {
    //     // invoke the default handler and exit the process
    //     orig_hook(panic_info);
    //     error!(logger_clone, "{:?}", Backtrace::new());
    //     process::exit(1);
    // }));

    // Listen on the AMF SCTP port so that when the worker starts up it will be able to connect.
    let amf_address = "127.0.0.1:38212";
    let server_stop_source = StopSource::new();
    let server_stop_token = server_stop_source.token();
    let server = SctpServerTransportProvider::new(NGAP_SCTP_PPID);
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
    let (worker_stop_source, worker_task) = worker::spawn(logger.new(o!("nodetype"=> "cu-w")));

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
    let precanned_ng_setup_response = hex::decode("20150031000004000100050100414d4600600008000002f839cafe0000564001ff005000100002f83900011008010203100811223300").unwrap();
    server
        .send_message(precanned_ng_setup_response, &logger)
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
