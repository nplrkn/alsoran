//! worker - the top level struct for a gNB-CU-UP worker, which implements the GnbCuUp trait

use crate::config::Config;
use crate::handlers::E1apHandler;
use crate::GnbCuUp;
use anyhow::Result;
use net::{SctpTransportProvider, ShutdownHandle, Stack};
use slog::{info, Logger};
use stop_token::{StopSource, StopToken};

#[derive(Clone)]
pub struct Worker {
    config: Config,
    e1ap: Stack,
    logger: Logger,
}

// TS38.462
const E1AP_SCTP_PPID: u32 = 64;
const E1AP_BIND_PORT: u16 = 38462;

pub fn spawn(config: Config, logger: Logger) -> Result<ShutdownHandle> {
    let stop_source = StopSource::new();
    let stop_token = stop_source.token();
    let worker = Worker::new(config, logger.clone());

    let handle = async_std::task::spawn(async move {
        worker
            .run(stop_token)
            .await
            .expect("Worker startup failure");
    });
    info!(&logger, "Started gNB-CU-UP worker");

    Ok(ShutdownHandle::new(handle, stop_source))
}

impl Worker {
    fn new(config: Config, logger: Logger) -> Worker {
        Worker {
            config,
            e1ap: Stack::new(SctpTransportProvider::new()),
            logger,
        }
    }

    async fn run(self, stop_token: StopToken) -> Result<()> {
        let cp_address = format!("{}:{}", &self.config.cp_ip_address, E1AP_BIND_PORT);

        self.e1ap
            .connect(
                &cp_address,
                "0.0.0.0",
                E1AP_SCTP_PPID,
                E1apHandler::new_e1ap_application(self.clone()),
                self.logger.clone(),
            )
            .await?;

        stop_token.await;

        self.e1ap.graceful_shutdown().await;

        Ok(())
    }
}

impl GnbCuUp for Worker {
    fn config(&self) -> &Config {
        &self.config
    }
}
