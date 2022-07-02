use anyhow::Result;
use async_std::task::JoinHandle;
use gnbcu::{Config, Gnbcu};
use mocks::MockAmf;
use mocks::MockDu;
use slog::{info, o, trace, Logger};
use std::{panic, process};
use stop_token::StopSource;

pub struct TestContext {
    pub amf: MockAmf,
    pub du: MockDu,
    du_stop_source: StopSource,
    pub logger: Logger,
    //coord_stop_source: StopSource,
    //coord_task: JoinHandle<()>,
    //control_task: JoinHandle<()>,
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

#[derive(PartialEq, PartialOrd)]
pub enum Stage {
    Init,
    AmfConnected,
    DuConnected,
    UeRegistered,
}

impl TestContext {
    pub async fn new(stage: Stage) -> Result<Self> {
        let logger = common::logging::test_init();

        let orig_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            orig_hook(panic_info);
            process::exit(1);
        }));

        // Listen on the AMF SCTP port so that when the worker starts up it will be able to connect.
        let amf_address = "127.0.0.1:38412";
        let amf = MockAmf::new(amf_address, &logger).await;

        let (du, du_stop_source) = MockDu::new(&logger).await;

        // let (coord_stop_source, coord_task, control_task) = coordinator::spawn(logger.new(o!("cu-c" => 1))).unwrap();
        let mut tc = TestContext {
            amf,
            du,
            du_stop_source,
            logger,
            //coord_stop_source,
            //coord_task,
            //control_task,
            workers: vec![],
        };
        tc.start_worker().await;

        tc.get_to_stage(stage).await
    }

    async fn get_to_stage(self, stage: Stage) -> Result<Self> {
        if stage >= Stage::AmfConnected {
            self.amf.expect_connection().await;
            self.amf.handle_ng_setup().await?;
        }
        if stage >= Stage::DuConnected {
            self.du
                .establish_connection(self.worker_info(0).f1ap_host_port)
                .await?;
            self.du.perform_f1_setup().await?;
        }
        if stage >= Stage::UeRegistered {
            self.du.perform_rrc_setup(Vec::new()).await?;
            self.amf.receive_initial_ue_message().await?;
            self.amf.send_initial_context_setup_request().await?;
            let security_mode_command = self.du.receive_ue_context_setup_request().await?;
            self.du.send_ue_context_setup_response().await?;
            self.du
                .send_security_mode_complete(&security_mode_command)
                .await?;
            let _nas = self.du.receive_rrc_reconfiguration().await?;
            self.du.send_rrc_reconfiguration_complete().await?;
            self.amf.receive_initial_context_setup_response().await?;
        }
        Ok(self)
    }

    pub fn worker_info(&self, index: usize) -> WorkerInfo {
        WorkerInfo {
            f1ap_host_port: format!("127.0.0.1:{}", self.workers[index].config.f1ap_bind_port),
        }
    }

    pub async fn start_worker(&mut self) {
        let worker_number = self.workers.len() as u16;

        let mut config = Config::default();
        //config.callback_server_bind_port += worker_number;
        config.f1ap_bind_port += worker_number;

        let (stop_source, task) =
            Gnbcu::spawn(config.clone(), &self.logger.new(o!("cu-w"=> worker_number))).unwrap();
        self.workers.push(InternalWorkerInfo {
            stop_source,
            task,
            config,
        })
    }

    pub async fn terminate(self) {
        //info!(self.logger, "Terminate coordinator");
        // drop(self.coord_stop_source);
        // self.coord_task.await;
        // self.control_task.await;

        trace!(self.logger, "Terminate workers");
        for worker in self.workers {
            drop(worker.stop_source);

            trace!(self.logger, "Wait for worker to terminate connection");
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
