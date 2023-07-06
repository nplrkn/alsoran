//! gnb_cu_up - the collection of services used by the GNB-CU-UP workflow business logic.
use crate::{config::Config, packet_processor::ForwardingAction};
use anyhow::Result;
use async_net::IpAddr;
use async_trait::async_trait;
use e1ap::GnbCuUpUeE1apId;
use net::{Procedure, RequestError};
use slog::Logger;
use xxap::GtpTeid;

/// Trait representing the collection of services needed by GNB-CU-UP workflows.
#[async_trait]
pub trait GnbCuUp: Send + Sync + Clone + 'static {
    fn config(&self) -> &Config;
    fn new_ue_ap_id(&self) -> GnbCuUpUeE1apId;
    fn create_uplink_teid(&self, ue_id: u32, session_id: u8) -> GtpTeid;
    fn create_downlink_teid(&self, ue_id: u32, session_id: u8) -> GtpTeid;
    async fn set_uplink_forwarding_action(&self, gtp_teid: GtpTeid, action: ForwardingAction);
    async fn set_downlink_forwarding_action(&self, gtp_teid: GtpTeid, action: ForwardingAction);
    fn bearer_context_exists(&self, ue_id: u32) -> bool;
    async fn delete_bearer_context(&self, ue_id: u32);
    async fn e1ap_connect(&self, cp_address: &IpAddr) -> Result<()>;
    async fn e1ap_request<P: Procedure>(
        &self,
        r: P::Request,
        logger: &Logger,
    ) -> Result<P::Success, RequestError<P::Failure>>;
}
