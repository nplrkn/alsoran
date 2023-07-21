//! worker - the top level struct for a gNB-CU-UP worker, which implements the GnbCuUp trait

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::handlers::E1apHandler;
use crate::packet_processor::ForwardingAction;
use crate::workflows::Workflow;
use crate::GnbCuUp;
use crate::{config::Config, packet_processor::PacketProcessor};
use anyhow::Result;
use asn1_per::{Procedure, RequestError, RequestProvider};
use async_net::IpAddr;
use async_trait::async_trait;
use dashmap::DashMap;
use e1ap::GnbCuUpUeE1apId;
use futures::{pin_mut, select, FutureExt};
use net::{SctpTransportProvider, ShutdownHandle, Stack};
use slog::{debug, info, warn, Logger};
use stop_token::{StopSource, StopToken};
use xxap::GtpTeid;

const RETRY_SECS: u64 = 10;
#[derive(Clone)]
pub struct Worker {
    config: Config,
    e1ap: Stack,
    packet_processor: PacketProcessor,
    logger: Logger,
    ue_ap_id_generator: Arc<AtomicU32>,
    ues: Arc<DashMap<u32, ()>>,
}

// TS38.462
const E1AP_SCTP_PPID: u32 = 64;
const E1AP_BIND_PORT: u16 = 38462;

pub async fn spawn(config: Config, logger: Logger) -> Result<ShutdownHandle> {
    let stop_source = StopSource::new();
    let stop_token = stop_source.token();
    info!(&logger, "Starting gNB-CU-UP worker");
    let worker = Worker::new(config, logger.clone()).await?;

    let handle = async_std::task::spawn(async move {
        worker
            .run(stop_token)
            .await
            .expect("Worker startup failure");
    });

    Ok(ShutdownHandle::new(handle, stop_source))
}

impl Worker {
    async fn new(config: Config, logger: Logger) -> Result<Worker> {
        let userplane_ip_address = config.userplane_ip_address;
        Ok(Worker {
            config,
            e1ap: Stack::new(SctpTransportProvider::new()),
            packet_processor: PacketProcessor::new(userplane_ip_address, logger.clone()).await?,
            logger,
            ue_ap_id_generator: Arc::new(AtomicU32::new(1)),
            ues: Arc::new(DashMap::new()),
        })
    }

    async fn run(self, stop_token: StopToken) -> Result<()> {
        let logger = &self.logger;

        // Infinitely retry to connect to GNB-CU-CP
        let stop_token = stop_token.fuse();
        pin_mut!(stop_token);
        loop {
            match Workflow::new(&self, &logger.clone())
                .gnb_cu_up_e1_setup(&self.config.cp_ip_address, 1, self.config.plmns())
                .await
            {
                Ok(_) => {
                    debug!(logger, "Startup complete - wait for instructions from CP");

                    // We now stay here for the lifetime of the GNB-CU-UP worker (until SIGINT)
                    // All the action happens in other tasks triggered by the E1apHandler.
                    stop_token.await;

                    break;
                }
                Err(e) => warn!(logger, "Connection to GNB-CU-CP failed - {}", e),
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
        Ok(())
    }
}

#[async_trait]
impl GnbCuUp for Worker {
    fn config(&self) -> &Config {
        &self.config
    }

    async fn set_uplink_forwarding_action(&self, gtp_teid: GtpTeid, action: ForwardingAction) {
        self.packet_processor
            .set_uplink_forwarding_action(gtp_teid, action, &self.logger)
            .await
    }

    async fn set_downlink_forwarding_action(&self, gtp_teid: GtpTeid, action: ForwardingAction) {
        self.packet_processor
            .set_downlink_forwarding_action(gtp_teid, action, &self.logger)
            .await
    }

    fn bearer_context_exists(&self, ue_id: u32) -> bool {
        self.ues.contains_key(&ue_id)
    }

    async fn delete_bearer_context(&self, ue_id: u32) {
        // Determine all the sessions for this UE and remove all the
        // forwarding actions

        // TODO: cope with > 1 session
        let uplink_teid_session_1 = self.create_uplink_teid(ue_id, 1);
        self.packet_processor
            .clear_forwarding_actions(uplink_teid_session_1)
            .await;
    }

    fn new_ue_ap_id(&self) -> GnbCuUpUeE1apId {
        let ue_id = GnbCuUpUeE1apId(self.ue_ap_id_generator.fetch_add(1, Ordering::Relaxed));
        self.ues.insert(ue_id.0, ());
        ue_id
    }

    // TODO - move to packet processor
    // TODO - this breaks the rule from 29.281, 5.1 of not using all 0s, and also breaks the
    // rule about predictability - given the CuUpE1apId from which it is generated is predictable.
    fn create_uplink_teid(&self, ue_id: u32, session_id: u8) -> GtpTeid {
        // 8 bits of session ID, the 23 LSBs of UE ID, 1 bit indicating direction
        GtpTeid([
            session_id,
            ((ue_id & 0x7f8000) >> 15) as u8,
            ((ue_id & 0x7f80) >> 7) as u8,
            ((ue_id & 0x7f) << 1) as u8, // LSB clear for uplink
        ])
    }

    fn create_downlink_teid(&self, ue_id: u32, session_id: u8) -> GtpTeid {
        // 8 bits of session ID, the 23 LSBs of UE ID, 1 bit indicating direction
        GtpTeid([
            session_id,
            ((ue_id & 0x7f8000) >> 15) as u8,
            ((ue_id & 0x7f80) >> 7) as u8,
            (((ue_id & 0x7f) << 1) | 1) as u8, // LSB set for downlink
        ])
    }

    async fn e1ap_connect(&self, cp_address: &IpAddr) -> Result<()> {
        let cp_address = format!("{}:{}", cp_address, E1AP_BIND_PORT);

        self.e1ap
            .connect(
                &cp_address,
                "0.0.0.0",
                E1AP_SCTP_PPID,
                E1apHandler::new_e1ap_application(self.clone()),
                self.logger.clone(),
            )
            .await
    }

    async fn e1ap_request<P: Procedure>(
        &self,
        r: P::Request,
        logger: &Logger,
    ) -> Result<P::Success, RequestError<P::Failure>> {
        <Stack as RequestProvider<P>>::request(&self.e1ap, r, logger)
            .await
            .map(|(x, _)| x)
    }
}
