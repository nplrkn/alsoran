use async_std;
use backtrace::Backtrace;
use common::sctp_server_transport_provider::SctpServerTransportProvider;
use common::transport_provider::{Handler, Message, ServerTransportProvider};
use slog::{error, info, o, Logger};
use std::panic;
use std::process;
use stop_token::StopSource;

#[derive(Debug, Clone)]
struct AmfHandler;

#[async_trait::async_trait]
impl Handler for AmfHandler {
    async fn recv_non_ue_associated(&self, m: Message, logger: &Logger) {
        info!(logger, "Got {:?}", m);
    }
}

const NGAP_SCTP_PPID: u32 = 60;

#[async_std::test]
async fn run_everything() {
    let logger = common::logging::test_init();

    let orig_hook = panic::take_hook();
    let logger_clone = logger.clone();
    panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        error!(logger_clone, "{:?}", Backtrace::new());
        process::exit(1);
    }));

    // Listen on the AMF SCTP port so that when the worker starts up it will be able to connect.
    let amf_address = "127.0.0.1:38212";
    let server_stop_source = StopSource::new();
    let server_stop_token = server_stop_source.token();
    let server = SctpServerTransportProvider::new(NGAP_SCTP_PPID);
    let amf_handler = AmfHandler {};
    let _server_task = server
        .serve(
            amf_address.to_string(),
            server_stop_token,
            amf_handler,
            logger.new(o!("nodetype"=> "amf")),
        )
        .await
        .expect("Server bind failed");

    let (coord_stop_source, coord_task) = coordinator::spawn(logger.new(o!("nodetype"=> "cu-c")));
    let (worker_stop_source, worker_task) = worker::spawn(logger.new(o!("nodetype"=> "cu-w")));

    async_std::task::sleep(std::time::Duration::from_secs(5)).await;

    drop(coord_stop_source);
    drop(worker_stop_source);
    drop(server_stop_source);
    coord_task.await;
    worker_task.await;
    //server_task.await;  // TODO
    drop(logger);
}
