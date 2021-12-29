use super::mock_amf::MockAmf;
use also_net::JsonCodec;
use async_std::task::JoinHandle;
use slog::{info, o, Logger};
use std::{panic, process};
use stop_token::StopSource;

pub struct TestContext {
    pub amf: MockAmf,
    pub logger: Logger,
    coord_stop_source: StopSource,
    coord_task: JoinHandle<()>,
    worker_stop_source: StopSource,
    worker_task: JoinHandle<()>,
}

impl TestContext {
    pub async fn new() -> Self {
        let logger = common::logging::test_init();

        let orig_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            orig_hook(panic_info);
            process::exit(1);
        }));

        // Listen on the AMF SCTP port so that when the worker starts up it will be able to connect.
        let amf_address = "127.0.0.1:38212";
        let amf = MockAmf::new(amf_address, &logger).await;

        let (coord_stop_source, coord_task) =
            coordinator::spawn(logger.new(o!("nodetype"=> "cu-c")));
        let (worker_stop_source, worker_task) = worker::spawn(
            logger.new(o!("nodetype"=> "cu-w")),
            JsonCodec::new(),
            JsonCodec::new(),
        );
        TestContext {
            amf,
            logger,
            coord_stop_source,
            coord_task,
            worker_stop_source,
            worker_task,
        }
    }

    pub async fn terminate(self) {
        info!(self.logger, "Terminate coordinator");
        drop(self.coord_stop_source);

        info!(self.logger, "Terminate worker");
        drop(self.worker_stop_source);

        info!(self.logger, "Wait for worker to terminate connection");
        assert!(self
            .amf
            .receiver
            .recv()
            .await
            .expect("Expected connection termination")
            .is_none());

        info!(self.logger, "Terminate mock AMF");
        drop(self.amf.stop_source);

        info!(self.logger, "Wait for all tasks to terminate cleanly");
        self.coord_task.await;
        self.worker_task.await;
        self.amf.task.await;
    }
}
