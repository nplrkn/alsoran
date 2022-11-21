use anyhow::{anyhow, Result};
use common::ShutdownHandle;
use coordinator::Config as CoordinatorConfig;
use gnbcu::{Config, ConnectionControlConfig, ConnectionStyle, WorkerConnectionManagementConfig};
use gnbcu::{MockUeStore, RedisUeStore};
use mocks::{
    AmfUeContext, CuUpUeContext, DuUeContext, MockAmf, MockCuUp, MockDu, SecurityModeCommand,
};
use rand::Rng;
use slog::{debug, info, o, warn, Logger};
use std::{panic, process};

const PORT_ALLOCATION_RETRIES: u32 = 10;
const CONNECTION_API_PORT: u16 = 50312;

pub struct TestContext {
    pub amf: MockAmf,
    pub amf_ip_addr: String,
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
    pub config: CoordinatorConfig,
}
#[derive(PartialEq, Eq, PartialOrd, Debug)]
pub enum Stage {
    Init,
    AmfConnected,
    CuUpConnected,
    DuConnected,
}

pub struct UeContext {
    ue_id: u32,
    du_ue_context: DuUeContext,
    amf_ue_context: Option<AmfUeContext>,
    cu_up_ue_context: Option<CuUpUeContext>,
}

pub struct UeRegister<'a> {
    pub stage: UeRegisterStage,
    ue_context: &'a mut UeContext,
}
pub enum UeRegisterStage {
    Init,
    Stage1(SecurityModeCommand),
    End,
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

    pub fn redis_port(&mut self, port: u16) -> &mut TestContextBuilder {
        self.redis_port = Some(port);
        self
    }

    pub fn stage(&mut self, stage: Stage) -> &mut TestContextBuilder {
        self.stage = stage;
        self
    }

    pub fn worker_count(&mut self, worker_count: isize) -> &mut TestContextBuilder {
        self.worker_count = worker_count;
        self
    }

    pub async fn spawn(&self) -> Result<TestContext> {
        let logger = common::logging::test_init();

        // Exit on panic
        let orig_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            orig_hook(panic_info);
            process::exit(1);
        }));

        // Listen on the AMF SCTP port so that when the worker starts up it will be able to connect.
        let (amf, amf_ip_addr) = start_amf_on_random_ip(&logger).await;
        let du = MockDu::new(&logger).await;
        let cu_up = MockCuUp::new(&logger).await;

        let mut tc = TestContext {
            amf,
            amf_ip_addr,
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

        // Maybe create a mock datastore to be shared by the workers (unless we're doing a live Redis test).
        let datastore = if let Some(port) = self.redis_port {
            WorkerDatastoreSetup::RedisPort(port)
        } else {
            WorkerDatastoreSetup::MockUeStore(MockUeStore::new())
        };

        // Start workers
        info!(tc.logger, "Spawn {} worker(s)", self.worker_count);
        for worker_index in 0..self.worker_count {
            tc.start_worker_on_random_ip(&datastore).await;
            tc.get_worker_to_stage(worker_index as usize, &self.stage)
                .await?;
        }

        Ok(tc)
    }
}

impl Default for TestContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub enum WorkerDatastoreSetup {
    RedisPort(u16),
    MockUeStore(MockUeStore),
}

impl TestContext {
    async fn start_coordinator(&mut self) {
        info!(self.logger, "Spawn coordinator");
        for _ in 0..PORT_ALLOCATION_RETRIES {
            let logger = self.logger.new(o!("cu-cp-coord" => 1));
            let config = CoordinatorConfig {
                bind_port: rand::thread_rng().gen_range(1024..65535),
                connection_control_config: ConnectionControlConfig {
                    amf_address: self.amf_ip_addr.clone(),
                    worker_refresh_interval_secs: 30,
                    fast_start: true,
                },
            };
            if let Ok(shutdown_handle) = coordinator::spawn(config.clone(), logger) {
                self.coordinator = Some(InternalCoordinatorInfo {
                    shutdown_handle,
                    config,
                });
                return;
            }
        }
        panic!("Repeatedly failed to start coordinator with random port");
    }

    async fn start_worker_on_random_ip(&mut self, datastore: &WorkerDatastoreSetup) {
        for _ in 0..PORT_ALLOCATION_RETRIES {
            let worker_ip = random_local_ip();
            let connection_api_bind_port = CONNECTION_API_PORT;

            let connection_style = if let Some(ref coordinator) = self.coordinator {
                ConnectionStyle::Coordinated(WorkerConnectionManagementConfig {
                    connection_api_bind_port,
                    connection_api_base_path: format!(
                        "http://{}:{}",
                        worker_ip, connection_api_bind_port
                    ),
                    coordinator_base_path: format!(
                        "http://127.0.0.1:{}",
                        coordinator.config.bind_port
                    ),
                })
            } else {
                ConnectionStyle::Autonomous(ConnectionControlConfig {
                    fast_start: true,
                    amf_address: self.amf_ip_addr.clone(),
                    ..ConnectionControlConfig::default()
                })
            };

            let config = Config {
                ip_addr: Some(worker_ip.parse().unwrap()),
                connection_style: connection_style.clone(),
                ..Config::default()
            };

            debug!(self.logger, "Start worker with config {:?}", config);

            match match datastore {
                WorkerDatastoreSetup::RedisPort(port) => gnbcu::spawn(
                    config.clone(),
                    RedisUeStore::new(*port).unwrap(),
                    self.logger.clone(),
                ),
                WorkerDatastoreSetup::MockUeStore(ue_store) => {
                    gnbcu::spawn(config.clone(), ue_store.clone(), self.logger.clone())
                }
            } {
                Ok(shutdown_handle) => {
                    self.workers.push(InternalWorkerInfo {
                        shutdown_handle,
                        config,
                    });
                    return;
                }
                Err(e) => warn!(self.logger, "Worker creation failed - {}", e),
            }
        }
        panic!("Repeatedly failed to create worker")
    }

    async fn get_worker_to_stage<'a>(
        &'a mut self,
        worker_index: usize,
        stage: &Stage,
    ) -> Result<&'a mut Self> {
        debug!(
            self.logger,
            "Get worker {} to stage {:?}", worker_index, stage
        );

        let worker_ip = self.workers[worker_index]
            .config
            .ip_addr
            .unwrap()
            .to_string();

        if stage >= &Stage::AmfConnected {
            self.amf.expect_connection().await;
            if worker_index == 0 {
                self.amf.handle_ng_setup().await?;
            } else {
                self.amf.handle_ran_configuration_update().await?;
            }
        }
        if stage >= &Stage::CuUpConnected {
            if worker_index == 0 {
                self.cu_up.perform_e1_setup(&worker_ip).await?;
            } else {
                self.cu_up
                    .handle_cu_cp_configuration_update(&worker_ip)
                    .await?;
            }
        }
        if stage >= &Stage::DuConnected {
            if worker_index == 0 {
                self.du.perform_f1_setup(&worker_ip).await?;
            } else {
                self.du.handle_cu_configuration_update(&worker_ip).await?;
            }
        }
        Ok(self)
    }

    pub async fn new_ue(&self, ue_id: u32) -> UeContext {
        UeContext {
            ue_id,
            du_ue_context: self.du.new_ue_context(ue_id).await,
            amf_ue_context: None,
            cu_up_ue_context: None,
        }
    }

    pub async fn create_and_register_ue(&self, ue_id: u32) -> Result<UeContext> {
        let mut ue_context = self.new_ue(ue_id).await;
        let mut register_ue = self.register_ue_start(&mut ue_context).await;
        loop {
            if let UeRegisterStage::End = register_ue.stage {
                break;
            }
            register_ue = self.register_ue_next(register_ue).await?;
        }
        Ok(ue_context)
    }

    pub async fn register_ue(&mut self, ue_context: &mut UeContext) -> Result<()> {
        let mut register_ue = self.register_ue_start(ue_context).await;
        loop {
            if let UeRegisterStage::End = register_ue.stage {
                break;
            }
            register_ue = self.register_ue_next(register_ue).await?;
        }
        Ok(())
    }

    pub async fn register_ue_start<'a>(&self, ue_context: &'a mut UeContext) -> UeRegister<'a> {
        info!(self.logger, "Register UE {}", ue_context.ue_id);
        UeRegister {
            stage: UeRegisterStage::Init,
            ue_context,
        }
    }

    pub async fn register_ue_next<'a>(
        &self,
        mut ue_register: UeRegister<'a>,
    ) -> Result<UeRegister<'a>> {
        let ue_id = ue_register.ue_context.ue_id;
        ue_register.stage = match &ue_register.stage {
            UeRegisterStage::Init => {
                self.du
                    .perform_rrc_setup(&mut ue_register.ue_context.du_ue_context, Vec::new())
                    .await
                    .unwrap();
                let amf_ue_context = self.amf.receive_initial_ue_message(ue_id).await.unwrap();
                self.amf
                    .send_initial_context_setup_request(&amf_ue_context)
                    .await
                    .unwrap();
                let security_mode_command = self.du.receive_security_mode_command(ue_id).await?;
                ue_register.ue_context.amf_ue_context = Some(amf_ue_context);
                UeRegisterStage::Stage1(security_mode_command)
            }
            UeRegisterStage::Stage1(security_mode_command) => {
                self.du
                    .send_security_mode_complete(
                        &ue_register.ue_context.du_ue_context,
                        security_mode_command,
                    )
                    .await
                    .unwrap();
                let amf_ue_context = ue_register.ue_context.amf_ue_context.as_ref().unwrap();
                self.amf
                    .receive_initial_context_setup_response(amf_ue_context)
                    .await
                    .unwrap();
                self.du.receive_nas(ue_id).await.unwrap();
                info!(self.logger, "Register UE {} complete", ue_id);
                UeRegisterStage::End
            }
            UeRegisterStage::End => return Err(anyhow!("Do not call in state End")),
        };
        Ok(ue_register)
    }

    // pub async fn register_ue_end(&self, ue_register: UeRegister) -> UeContext {
    //     ue_register.ue_context
    // }

    pub async fn establish_pdu_session(&mut self, ue_context: &mut UeContext) -> Result<()> {
        let ue_id = ue_context.ue_id;
        let amf_ue_context = ue_context.amf_ue_context.as_ref().unwrap();

        info!(self.logger, "Establish PDU session for UE {}", ue_id);
        self.amf
            .send_pdu_session_resource_setup(amf_ue_context)
            .await
            .unwrap();
        let cu_up_ue_context = self.cu_up.handle_bearer_context_setup(ue_id).await.unwrap();
        self.du
            .handle_ue_context_setup(&ue_context.du_ue_context)
            .await
            .unwrap();
        self.cu_up
            .handle_bearer_context_modification(&cu_up_ue_context)
            .await
            .unwrap();
        ue_context.cu_up_ue_context = Some(cu_up_ue_context);
        let _nas = self.du.receive_rrc_reconfiguration(ue_id).await.unwrap();
        self.du
            .send_rrc_reconfiguration_complete(&ue_context.du_ue_context)
            .await
            .unwrap();
        self.amf
            .receive_pdu_session_resource_setup_response(amf_ue_context)
            .await
            .unwrap();
        info!(
            self.logger,
            "Finished establishing PDU session for UE {}", ue_id
        );
        Ok(())
    }

    pub async fn terminate(self) {
        info!(self.logger, "Terminate worker(s)");
        for worker in self.workers {
            worker.shutdown_handle.graceful_shutdown().await;
            // We don't know if the worker has a connection up, so we can't assume we will see a connection
            // hangup on the AMF.
            //self.amf.expect_connection().await;
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

async fn start_amf_on_random_ip(logger: &Logger) -> (MockAmf, String) {
    for _ in 0..PORT_ALLOCATION_RETRIES {
        let address = random_local_ip();
        if let Ok(amf) = MockAmf::new(&address, logger).await {
            return (amf, address);
        };
    }
    panic!("Repeatedly failed to start Mock AMF")
}

fn random_local_ip() -> String {
    format!(
        "127.{}.{}.{}",
        rand::thread_rng().gen_range(0..255),
        rand::thread_rng().gen_range(0..255),
        rand::thread_rng().gen_range(1..255)
    )
}
