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
    control_task: JoinHandle<()>,
    workers: Vec<WorkerInfo>,
}

struct WorkerInfo {
    pub stop_source: StopSource,
    pub task: JoinHandle<()>,
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

        let (coord_stop_source, coord_task, control_task) =
            coordinator::spawn(logger.new(o!("nodetype"=> "cu-c")));
        let mut tc = TestContext {
            amf,
            logger,
            coord_stop_source,
            coord_task,
            control_task,
            workers: vec![],
        };
        tc.start_worker().await;
        tc
    }

    pub async fn start_worker(&mut self) {
        let (stop_source, task) = worker::spawn(
            self.logger.new(o!("cu-w"=> self.workers.len())),
            JsonCodec::new(),
            JsonCodec::new(),
        );
        self.workers.push(WorkerInfo { stop_source, task })
    }

    pub async fn terminate(self) {
        info!(self.logger, "Terminate coordinator");
        drop(self.coord_stop_source);

        info!(self.logger, "Terminate workers");
        for worker in self.workers {
            drop(worker.stop_source);

            info!(self.logger, "Wait for worker to terminate connection");
            assert!(self
                .amf
                .receiver
                .recv()
                .await
                .expect("Expected connection termination")
                .is_none());
            worker.task.await;
        }

        info!(self.logger, "Terminate mock AMF");
        drop(self.amf.stop_source);

        info!(self.logger, "Wait for all tasks to terminate cleanly");
        self.coord_task.await;
        self.control_task.await;
        self.amf.task.await;
    }
}
