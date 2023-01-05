use super::ue::*;
use anyhow::Result;
use async_std::future;
use common::ShutdownHandle;
use coordinator::Config as CoordinatorConfig;
use gnb_cu_cp::{
    Config, ConnectionControlConfig, ConnectionStyle, WorkerConnectionManagementConfig,
};
use gnb_cu_cp::{MockUeStore, RedisUeStore};
use mocks::{MockAmf, MockCuUp, MockDu};
use rand::Rng;
use slog::{debug, info, o, warn, Logger};
use std::time::Duration;
use std::{panic, process};
use uuid::Uuid;

const IP_OR_PORT_RETRIES: usize = 10;
const CONNECTION_API_PORT: u16 = 50312;

pub struct TestContext {
    pub amf: MockAmf,
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
    AmfSecondaryEndpointsConnected,
    CuUpConnected,
    DuConnected,
}

pub struct TestContextBuilder {
    redis_port: Option<u16>,
    stage: Stage,
    worker_count: usize,
    amf_endpoint_count: usize,
}

impl TestContextBuilder {
    pub fn new() -> Self {
        TestContextBuilder {
            redis_port: None,
            stage: Stage::Init,
            worker_count: 1,
            amf_endpoint_count: 1,
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

    pub fn worker_count(&mut self, worker_count: usize) -> &mut TestContextBuilder {
        self.worker_count = worker_count;
        self
    }

    pub fn amf_endpoint_count(&mut self, amf_endpoint_count: usize) -> &mut TestContextBuilder {
        self.amf_endpoint_count = amf_endpoint_count;
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
        let amf = start_amf_with_random_ips(&logger, self.amf_endpoint_count).await;
        let du = MockDu::new(&logger).await;
        let cu_up = MockCuUp::new(&logger).await;

        let mut tc = TestContext {
            amf,
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
            tc.get_worker_to_stage(worker_index as usize, &self.stage, worker_index == 0)
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
        for _ in 0..IP_OR_PORT_RETRIES {
            let logger = self.logger.new(o!("cu-cp-coord" => 1));
            let config = CoordinatorConfig {
                bind_port: rand::thread_rng().gen_range(1024..65535),
                connection_control_config: ConnectionControlConfig {
                    amf_address: self.amf.ips()[0].clone(),
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

    // TODO - factor out worker stuff to separate file
    async fn start_worker_on_random_ip(&mut self, datastore: &WorkerDatastoreSetup) {
        for _ in 0..IP_OR_PORT_RETRIES {
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
                    amf_address: self.amf.ips()[0].clone(),
                    ..ConnectionControlConfig::default()
                })
            };

            let config = Config {
                ip_addr: Some(worker_ip.parse().unwrap()),
                connection_style: connection_style.clone(),
                ..Config::default()
            };

            debug!(self.logger, "Start worker with config {:?}", config);

            // We allocate the worker ID and logger here rather than inside Worker::new() in order that we can give the
            // coordinator the same logger as the worker.
            let worker_id = Uuid::new_v4();
            let worker_logger = self.logger.new(o!("cu-cp-w"=> worker_id.to_string()));
            match match datastore {
                WorkerDatastoreSetup::RedisPort(port) => gnb_cu_cp::spawn(
                    worker_id,
                    config.clone(),
                    RedisUeStore::new(*port).unwrap(),
                    worker_logger,
                ),
                WorkerDatastoreSetup::MockUeStore(ue_store) => {
                    gnb_cu_cp::spawn(worker_id, config.clone(), ue_store.clone(), worker_logger)
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

    pub fn worker_ip(&self, worker_index: usize) -> String {
        let worker_index = worker_index % self.workers.len();
        self.workers[worker_index]
            .config
            .ip_addr
            .unwrap()
            .to_string()
    }

    pub async fn interface_setup_stage<'a>(
        &'a mut self,
        worker_index: usize,
        stage: &Stage,
        setup_interface: bool,
    ) -> Result<&'a mut Self> {
        let worker_ip = self.worker_ip(worker_index);

        match stage {
            &Stage::Init => (),
            &Stage::AmfConnected => {
                self.amf.expect_connection().await;
                if setup_interface {
                    self.amf.handle_ng_setup().await?;
                } else {
                    self.amf.handle_ran_configuration_update().await?;
                }
            }
            &Stage::AmfSecondaryEndpointsConnected => todo!(),
            &Stage::CuUpConnected => {
                if setup_interface {
                    self.cu_up.perform_e1_setup(&worker_ip).await?;
                } else {
                    self.cu_up
                        .handle_cu_cp_configuration_update(&worker_ip)
                        .await?;
                }
            }
            &Stage::DuConnected => {
                if setup_interface {
                    self.du.perform_f1_setup(&worker_ip).await?;
                } else {
                    self.du.handle_cu_configuration_update(&worker_ip).await?;
                }
            }
        }

        Ok(self)
    }

    async fn get_worker_to_stage<'a>(
        &'a mut self,
        worker_index: usize,
        stage: &Stage,
        setup_interface: bool,
    ) -> Result<&'a mut Self> {
        debug!(
            self.logger,
            "Get worker {} to stage {:?}", worker_index, stage
        );

        if stage >= &Stage::AmfConnected {
            self.interface_setup_stage(worker_index, &Stage::AmfConnected, setup_interface)
                .await?;
        }
        if stage >= &Stage::AmfSecondaryEndpointsConnected {
            if self.amf.ips().len() > 1 {
                todo!()
            }
        }
        if stage >= &Stage::CuUpConnected {
            self.interface_setup_stage(worker_index, &Stage::CuUpConnected, setup_interface)
                .await?;
        }
        if stage >= &Stage::DuConnected {
            self.interface_setup_stage(worker_index, &Stage::DuConnected, setup_interface)
                .await?;
        }
        Ok(self)
    }

    pub async fn new_ue(&self, ue_id: u32) -> Result<DetachedUe> {
        assert!(ue_id > 0);
        let worker_ip = self.worker_ip((ue_id - 1) as usize);
        let du_ue_context = self.du.new_ue_context(ue_id, &worker_ip).await?;
        Ok(DetachedUe::new(ue_id, du_ue_context))
    }

    pub async fn use_worker_for_ue<T: RebindUe>(
        &self,
        worker_index: usize,
        ue: &mut T,
    ) -> Result<()> {
        ue.rebind(&self, &self.worker_ip(worker_index)).await
    }

    pub async fn create_and_register_ue(&self, ue_id: u32) -> Result<RegisteredUe> {
        self.new_ue(ue_id)
            .await?
            .initial_access(self)
            .await?
            .initiate_registration(self)
            .await?
            .complete_registration(self)
            .await
    }

    async fn graceful_terminate(self) {
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

    pub async fn terminate(self) {
        info!(self.logger, "Terminate worker(s)");
        future::timeout(Duration::from_secs(10), self.graceful_terminate())
            .await
            .expect("Graceful shutdown took more than 10 seconds");
    }
}

async fn start_amf_with_random_ips(logger: &Logger, num_endpoints: usize) -> MockAmf {
    assert!(num_endpoints > 0);
    let mut amf = MockAmf::new(logger).await;

    for _ in 0..(IP_OR_PORT_RETRIES + num_endpoints) {
        let _ = amf.add_endpoint(&random_local_ip()).await;
        if amf.ips().len() == num_endpoints {
            return amf;
        }
    }
    panic!("Failed to bind to {} random IPs", num_endpoints)
}

fn random_local_ip() -> String {
    format!(
        "127.{}.{}.{}",
        rand::thread_rng().gen_range(0..255),
        rand::thread_rng().gen_range(0..255),
        rand::thread_rng().gen_range(1..255)
    )
}
