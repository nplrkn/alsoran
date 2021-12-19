use async_std;
use common::sctp_server_transport_provider::SctpServerTransportProvider;
use common::transport_provider::{Handler, Message, ServerTransportProvider};
use slog::{o, Logger};
use stop_token::StopSource;

#[derive(Debug, Clone)]
struct AmfHandler;

#[async_trait::async_trait]
impl Handler for AmfHandler {
    async fn recv_non_ue_associated(&self, _m: Message, _logger: &Logger) {
        unimplemented!()
    }
}

#[async_std::test]
async fn run_everything() {
    let logger = common::logging::test_init();

    // Listen on the AMF SCTP port so that when the worker starts up it will be able to connect.
    let amf_address = "127.0.0.1:3456";
    let stop_source = StopSource::new();
    let stop_token = stop_source.token();
    let server = SctpServerTransportProvider::new();
    let server_task = server
        .serve(
            amf_address.to_string(),
            stop_token,
            AmfHandler,
            logger.new(o!("nodetype"=> "amf")),
        )
        .await
        .expect("Server bind failed");

    let (coord_stop_source, coord_task) = coordinator::spawn(logger.new(o!("nodetype"=> "cu-c")));
    let (worker_stop_source, worker_task) = worker::spawn(logger.new(o!("nodetype"=> "cu-w")));

    async_std::task::sleep(std::time::Duration::from_secs(5)).await;

    drop(coord_stop_source);
    drop(worker_stop_source);
    coord_task.await;
    worker_task.await;
    server_task.await;
    drop(logger);
}
