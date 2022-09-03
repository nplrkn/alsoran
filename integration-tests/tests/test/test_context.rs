use anyhow::Result;
use common::ShutdownHandle;
use gnbcu::{ConcreteGnbcu, Config};
use gnbcu::{MockUeStore, RedisUeStore};
use mocks::{MockAmf, MockCuUp, MockDu};
use slog::{info, o, trace, Logger};
use std::{panic, process};

const F1AP_SCTP_PPID: u32 = 62;
const E1AP_SCTP_PPID: u32 = 64;

pub struct TestContext {
    pub amf: MockAmf,
    pub du: MockDu,
    pub cu_up: MockCuUp,
    pub logger: Logger,
    workers: Vec<InternalWorkerInfo>,
}

struct InternalWorkerInfo {
    pub shutdown_handle: ShutdownHandle,
    pub config: Config,
}

pub struct WorkerInfo {
    pub f1ap_host_port: String,
    pub e1ap_host_port: String,
}

#[derive(PartialEq, PartialOrd)]
pub enum Stage {
    Init,
    AmfConnected,
    DuConnected,
    CuUpConnected,
    Ue1Registered,
}

impl TestContext {
    pub async fn new(stage: Stage) -> Result<Self> {
        Self::new_with(stage, None).await
    }

    pub async fn new_with_redis(stage: Stage, redis_port: u16) -> Result<Self> {
        Self::new_with(stage, Some(redis_port)).await
    }

    async fn new_with(stage: Stage, redis: Option<u16>) -> Result<Self> {
        let logger = common::logging::test_init();

        let orig_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            orig_hook(panic_info);
            process::exit(1);
        }));

        // Listen on the AMF SCTP port so that when the worker starts up it will be able to connect.
        let amf_address = "127.0.0.1:38412";
        let amf = MockAmf::new(amf_address, &logger).await;
        let du = MockDu::new(&logger).await;
        let cu_up = MockCuUp::new(&logger).await;

        let mut tc = TestContext {
            amf,
            du,
            cu_up,
            logger,
            workers: vec![],
        };
        tc.start_worker(redis).await;

        tc.get_to_stage(stage).await
    }

    async fn get_to_stage(mut self, stage: Stage) -> Result<Self> {
        if stage >= Stage::AmfConnected {
            self.amf.expect_connection().await;
            self.amf.handle_ng_setup().await?;
        }
        if stage >= Stage::DuConnected {
            let address = self.worker_info(0).f1ap_host_port;
            self.du.connect(address, F1AP_SCTP_PPID).await;
            self.du.perform_f1_setup().await?;
        }
        if stage >= Stage::CuUpConnected {
            let address = self.worker_info(0).e1ap_host_port;
            self.cu_up.connect(address, E1AP_SCTP_PPID).await;
            self.cu_up.perform_e1_setup().await?;
        }
        if stage >= Stage::Ue1Registered {
            self.register_ue(1).await?;
        }
        Ok(self)
    }

    pub fn worker_info(&self, index: usize) -> WorkerInfo {
        WorkerInfo {
            f1ap_host_port: format!("127.0.0.1:{}", self.workers[index].config.f1ap_bind_port),
            e1ap_host_port: format!("127.0.0.1:{}", self.workers[index].config.e1ap_bind_port),
        }
    }

    pub async fn register_ue(&mut self, ue_id: u32) -> Result<()> {
        self.du.perform_rrc_setup(ue_id, Vec::new()).await?;
        self.amf.receive_initial_ue_message(ue_id).await?;
        self.amf.send_initial_context_setup_request(ue_id).await?;
        let security_mode_command = self.du.receive_ue_context_setup_request(ue_id).await?;
        self.du.send_ue_context_setup_response(ue_id).await?;
        self.du
            .send_security_mode_complete(ue_id, &security_mode_command)
            .await?;
        let _nas = self.du.receive_rrc_reconfiguration(ue_id).await?;
        self.du.send_rrc_reconfiguration_complete(ue_id).await?;
        self.amf
            .receive_initial_context_setup_response(ue_id)
            .await?;
        Ok(())
    }

    pub async fn start_worker(&mut self, redis_port: Option<u16>) {
        let worker_number = self.workers.len() as u16;

        let mut config = Config::default();
        config.f1ap_bind_port += worker_number;
        let logger = self.logger.new(o!("cu-w"=> worker_number));

        let shutdown_handle = if let Some(port) = redis_port {
            ConcreteGnbcu::spawn(config.clone(), RedisUeStore::new(port).unwrap(), &logger)
        } else {
            ConcreteGnbcu::spawn(config.clone(), MockUeStore::new(), &logger)
        }
        .unwrap();
        self.workers.push(InternalWorkerInfo {
            shutdown_handle,
            config,
        })
    }

    pub async fn terminate(self) {
        trace!(self.logger, "Terminate workers");
        for worker in self.workers {
            worker.shutdown_handle.graceful_shutdown().await;
            trace!(self.logger, "Wait for worker to terminate connection");
            self.amf.expect_connection().await;
        }

        info!(self.logger, "Terminate mock AMF");
        self.amf.terminate().await;

        info!(self.logger, "Terminate mock DU");
        self.du.terminate().await;
    }
}
