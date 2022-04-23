use super::mock_amf::MockAmf;
use super::mock_du::MockDu;
use async_std::task::JoinHandle;
use net::Asn1PerCodec;
use slog::{info, o, Logger};
use std::{panic, process};
use stop_token::StopSource;
use worker::Config;

pub struct TestContext {
    pub amf: MockAmf,
    pub du: MockDu,
    du_stop_source: StopSource,
    pub logger: Logger,
    coord_stop_source: StopSource,
    coord_task: JoinHandle<()>,
    control_task: JoinHandle<()>,
    workers: Vec<InternalWorkerInfo>,
}

struct InternalWorkerInfo {
    pub stop_source: StopSource,
    pub task: JoinHandle<()>,
    pub config: Config,
}

pub struct WorkerInfo {
    pub f1ap_host_port: String,
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

        let (du, du_stop_source) = MockDu::new(&logger).await;

        let (coord_stop_source, coord_task, control_task) =
            coordinator::spawn(logger.new(o!("cu-c" => 1))).unwrap();
        let mut tc = TestContext {
            amf,
            du,
            du_stop_source,
            logger,
            coord_stop_source,
            coord_task,
            control_task,
            workers: vec![],
        };
        tc.start_worker().await;
        tc
    }

    pub fn worker_info(&self, index: usize) -> WorkerInfo {
        WorkerInfo {
            f1ap_host_port: format!("127.0.0.1:{}", self.workers[index].config.f1ap_bind_port),
        }
    }

    pub async fn start_worker(&mut self) {
        let worker_number = self.workers.len() as u16;

        let mut config = Config::default();
        config.callback_server_bind_port += worker_number;
        config.f1ap_bind_port += worker_number;

        let (stop_source, task) = worker::spawn(
            config.clone(),
            self.logger.new(o!("cu-w"=> worker_number)),
            Asn1PerCodec::new(),
            Asn1PerCodec::new(),
        )
        .unwrap();
        self.workers.push(InternalWorkerInfo {
            stop_source,
            task,
            config,
        })
    }

    pub async fn terminate(self) {
        info!(self.logger, "Terminate coordinator");
        drop(self.coord_stop_source);
        self.coord_task.await;
        self.control_task.await;

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
        self.amf.task.await;

        info!(self.logger, "Terminate mock DU");
        drop(self.du_stop_source);
    }
}
