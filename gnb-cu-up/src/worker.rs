//! worker - the top level struct for a gNB-CU-UP worker, which implements the GnbCuUp trait

use std::time::Duration;

use crate::config::Config;
use crate::handlers::E1apHandler;
use crate::GnbCuUp;
use anyhow::Result;
use futures::{pin_mut, select, FutureExt};
use net::{SctpTransportProvider, ShutdownHandle, Stack};
use slog::{info, warn, Logger};
use stop_token::{StopSource, StopToken};

const RETRY_SECS: u64 = 10;
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
        let logger = &self.logger;

        // Infinitely retry to connect to GNB-CU-CP
        let cp_address = format!("{}:{}", &self.config.cp_ip_address, E1AP_BIND_PORT);
        let stop_token = stop_token.fuse();
        pin_mut!(stop_token);
        loop {
            let attempt = self
                .e1ap
                .connect(
                    &cp_address,
                    "0.0.0.0",
                    E1AP_SCTP_PPID,
                    E1apHandler::new_e1ap_application(self.clone()),
                    self.logger.clone(),
                )
                .fuse();
            pin_mut!(attempt);
            select! {
            result = attempt => match result {
                Ok(_) => {
                    info!(logger, "Startup complete - wait for instructions from CP");

                    // We now stay here for the lifetime of the GNB-CU-UP worker (until SIGINT)
                    // All the action happens in other tasks triggered by the E1apHandler.
                    stop_token.await;

                    break;
                },
                Err(e) => warn!(logger, "Connection to GNB-CU-CP {} failed - error: {}", cp_address, e)
            },

            // Stopped while waiting for connection - break out and shut down
            _ = stop_token => break
            }

            info!(logger, "Pausing for {}s before retry", RETRY_SECS);
            select! {
                // Stopped while waiting for connection retry timer - break out and shut down
                _ = stop_token => break,

                // Connection retry timer popped - continue loop and try again.
                _ = async_std::task::sleep(Duration::from_secs(RETRY_SECS)).fuse() => ()
            }
        }
        self.e1ap.graceful_shutdown().await;
        return Ok(());
    }
}

impl GnbCuUp for Worker {
    fn config(&self) -> &Config {
        &self.config
    }
}
