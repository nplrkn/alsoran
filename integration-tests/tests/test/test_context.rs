use anyhow::Result;
use common::ShutdownHandle;
use coordinator::{Config as CoordinatorConfig, Coordinator};
use gnbcu::{ConcreteGnbcu, Config};
use gnbcu::{MockUeStore, RedisUeStore};
use mocks::{MockAmf, MockCuUp, MockDu, SecurityModeCommand};
use rand::Rng;
use slog::{info, o, Logger};
use std::{panic, process};

const F1AP_SCTP_PPID: u32 = 62;
const E1AP_SCTP_PPID: u32 = 64;
const PORT_ALLOCATION_RETRIES: u32 = 10;

pub struct TestContext {
    pub amf: MockAmf,
    pub amf_port: u16,
    pub du: MockDu,
    pub cu_up: MockCuUp,
    pub logger: Logger,
    workers: Vec<InternalWorkerInfo>,
    coordinator: Option<InternalCoordinatorInfo>,
}

struct InternalWorkerInfo {
    pub shutdown_handle: ShutdownHandle,
    pub config: Config,
}

struct InternalCoordinatorInfo {
    pub shutdown_handle: ShutdownHandle,
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

pub struct UeRegister {
    pub stage: UeRegisterStage,
    ue_id: u32,
}
pub enum UeRegisterStage {
    Init,
    Stage1(SecurityModeCommand),
}

pub struct TestContextBuilder {
    redis_port: Option<u16>,
    stage: Stage,
    worker_count: isize,
}

impl TestContextBuilder {
    pub fn new() -> Self {
        TestContextBuilder {
            redis_port: None,
            stage: Stage::Init,
            worker_count: 1,
        }
    }

    pub fn redis_port<'a>(&'a mut self, port: u16) -> &'a mut TestContextBuilder {
        self.redis_port = Some(port);
        self
    }

    pub fn stage<'a>(&'a mut self, stage: Stage) -> &'a mut TestContextBuilder {
        self.stage = stage;
        self
    }

    pub fn worker_count<'a>(&'a mut self, worker_count: isize) -> &'a mut TestContextBuilder {
        self.worker_count = worker_count;
        self
    }

    pub async fn spawn(&self) -> Result<TestContext> {
        let logger = common::logging::test_init();

        let orig_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            orig_hook(panic_info);
            process::exit(1);
        }));

        // Listen on the AMF SCTP port so that when the worker starts up it will be able to connect.
        let (amf, amf_port) = start_amf_on_random_port(&logger).await;
        let du = MockDu::new(&logger).await;
        let cu_up = MockCuUp::new(&logger).await;

        let mut tc = TestContext {
            amf,
            amf_port,
            du,
            cu_up,
            logger,
            workers: vec![],
            coordinator: None,
        };

        // Start coordinator if there will be multiple workers.
        if self.worker_count > 1 {
            tc.start_coordinator().await;
        }

        // Start workers
        info!(tc.logger, "Spawn {} workers", self.worker_count);
        for _ in 0..self.worker_count {
            tc.start_worker_with_random_ports(self.redis_port).await;
        }

        tc.get_to_stage(&self.stage).await?;
        Ok(tc)
    }
}

impl TestContext {
    async fn start_coordinator(&mut self) {
        info!(self.logger, "Spawn coordinator");
        for _ in 0..PORT_ALLOCATION_RETRIES {
            let logger = self.logger.new(o!("coord" => 1));
            let config = CoordinatorConfig {
                bind_port: rand::thread_rng().gen_range(1024..65535),
            };
            if let Ok(shutdown_handle) = Coordinator::spawn(config, logger) {
                self.coordinator = Some(InternalCoordinatorInfo { shutdown_handle });
                return;
            }
        }
        panic!("Repeatedly failed to start coordinator with random port");
    }

    async fn start_worker_with_random_ports(&mut self, redis_port: Option<u16>) {
        let worker_number = self.workers.len() as u16;
        let logger = self.logger.new(o!("cu-w"=> worker_number));
        for _ in 0..PORT_ALLOCATION_RETRIES {
            let mut config = Config::default();
            config.amf_address = format!("127.0.0.1:{}", self.amf_port);
            config.f1ap_bind_port = rand::thread_rng().gen_range(1024..65535);
            config.e1ap_bind_port = config.f1ap_bind_port + 1;

            if let Ok(shutdown_handle) = if let Some(port) = redis_port {
                ConcreteGnbcu::spawn(config.clone(), RedisUeStore::new(port).unwrap(), &logger)
            } else {
                ConcreteGnbcu::spawn(config.clone(), MockUeStore::new(), &logger)
            } {
                self.workers.push(InternalWorkerInfo {
                    shutdown_handle,
                    config,
                });
                return;
            }
        }
        panic!("Repeatedly failed to start worker with random ports");
    }

    async fn get_to_stage<'a>(&'a mut self, stage: &Stage) -> Result<&'a mut Self> {
        if stage >= &Stage::AmfConnected {
            self.amf.expect_connection().await;
            self.amf.handle_ng_setup().await?;
        }
        if stage >= &Stage::DuConnected {
            let address = self.worker_info(0).f1ap_host_port;
            self.du.connect(&address, F1AP_SCTP_PPID).await;
            self.du.perform_f1_setup().await?;
        }
        if stage >= &Stage::CuUpConnected {
            let address = self.worker_info(0).e1ap_host_port;
            self.cu_up.connect(&address, E1AP_SCTP_PPID).await;
            self.cu_up.perform_e1_setup().await?;
        }
        if stage >= &Stage::Ue1Registered {
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
        let mut register_ue = self.register_ue_start(ue_id);
        loop {
            if let Some(x) = self.register_ue_next(register_ue).await? {
                register_ue = x
            } else {
                return Ok(());
            }
        }
    }

    pub fn register_ue_start(&mut self, ue_id: u32) -> UeRegister {
        UeRegister {
            stage: UeRegisterStage::Init,
            ue_id,
        }
    }

    pub async fn register_ue_next(
        &mut self,
        ue_register: UeRegister,
    ) -> Result<Option<UeRegister>> {
        let ue_id = ue_register.ue_id;
        let stage = match ue_register.stage {
            UeRegisterStage::Init => {
                self.du.perform_rrc_setup(ue_id, Vec::new()).await.unwrap();
                self.amf.receive_initial_ue_message(ue_id).await.unwrap();
                self.amf
                    .send_initial_context_setup_request(ue_id)
                    .await
                    .unwrap();
                let security_mode_command = self.du.receive_security_mode_command(ue_id).await?;
                UeRegisterStage::Stage1(security_mode_command)
            }
            UeRegisterStage::Stage1(security_mode_command) => {
                self.du
                    .send_security_mode_complete(ue_id, &security_mode_command)
                    .await
                    .unwrap();
                self.amf
                    .receive_initial_context_setup_response(ue_id)
                    .await
                    .unwrap();
                self.du.receive_nas(ue_id).await.unwrap();
                return Ok(None);
            }
        };
        Ok(Some(UeRegister { ue_id, stage }))
    }

    pub async fn establish_pdu_session(&mut self, ue_id: u32) -> Result<()> {
        self.amf
            .send_pdu_session_resource_setup(ue_id)
            .await
            .unwrap();
        self.cu_up.handle_bearer_context_setup(ue_id).await.unwrap();
        self.du.handle_ue_context_setup(ue_id).await.unwrap();
        self.cu_up
            .handle_bearer_context_modification(ue_id)
            .await
            .unwrap();
        let _nas = self.du.receive_rrc_reconfiguration(ue_id).await.unwrap();
        self.du
            .send_rrc_reconfiguration_complete(ue_id)
            .await
            .unwrap();
        self.amf
            .receive_pdu_session_resource_setup_response(ue_id)
            .await
    }

    pub async fn terminate(self) {
        info!(self.logger, "Terminate workers");
        for worker in self.workers {
            worker.shutdown_handle.graceful_shutdown().await;
            self.amf.expect_connection().await;
        }

        if let Some(c) = self.coordinator {
            info!(self.logger, "Terminate coordinator");
            c.shutdown_handle.graceful_shutdown().await;
        }

        info!(self.logger, "Terminate mock AMF");
        self.amf.terminate().await;

        info!(self.logger, "Terminate mock DU");
        self.du.terminate().await;
    }
}

async fn start_amf_on_random_port(logger: &Logger) -> (MockAmf, u16) {
    for _ in 0..PORT_ALLOCATION_RETRIES {
        let port = rand::thread_rng().gen_range(1024..65535);
        let address = format!("127.0.0.1:{}", port);

        if let Ok(amf) = MockAmf::new(address.as_str(), &logger).await {
            return (amf, port);
        };
    }
    panic!("Repeatedly failed to start Mock AMF")
}
