//! gnbcu_trait - the collection of services used by the GNB-CU workflow business logic.

use super::Config;
use crate::{
    datastore::{UeState, UeStateStore},
    rrc_transaction::RrcTransaction,
};
use anyhow::Result;
use async_channel::Sender;
use async_trait::async_trait;
use net::{Indication, Procedure, RequestError};
use rrc::UlDcchMessage;
use slog::Logger;

/// Gnbcu trait representing the collection of services needed by Gnbcu procedure logic.
#[async_trait]
pub trait Gnbcu: Send + Sync + Clone + 'static + UeStateStore {
    fn config(&self) -> &Config;

    async fn ngap_connect(&self, amf_address: &str) -> Result<()>;

    async fn ngap_request<P: Procedure>(
        &self,
        r: P::Request,
        logger: &Logger,
    ) -> Result<P::Success, RequestError<P::Failure>>;
    async fn ngap_indication<P: Indication>(&self, r: P::Request, logger: &Logger);

    async fn f1ap_request<P: Procedure>(
        &self,
        r: P::Request,
        logger: &Logger,
    ) -> Result<P::Success, RequestError<P::Failure>>;
    async fn f1ap_indication<P: Indication>(&self, r: P::Request, logger: &Logger);

    async fn e1ap_request<P: Procedure>(
        &self,
        r: P::Request,
        logger: &Logger,
    ) -> Result<P::Success, RequestError<P::Failure>>;

    async fn e1ap_indication<P: Indication>(&self, r: P::Request, logger: &Logger);

    // TODO - make RRC request and indication similar to the above?
    // See "This was an idea for a more elegant model" in initial_access.rs.

    /// Start a new RRC transaction.
    async fn new_rrc_transaction(&self, ue: &UeState) -> RrcTransaction;

    /// Determine if this is a response to a local pending RRC transaction.
    async fn match_rrc_transaction(&self, ue_id: u32) -> Option<Sender<UlDcchMessage>>;

    async fn send_rrc_to_ue(
        &self,
        ue: &UeState,
        srb_id: f1ap::SrbId,
        rrc_container: f1ap::RrcContainer,
        logger: &Logger,
    );

    /// Associate a TNLA with the relevant interface instance.  For example, an NG Setup
    /// associates a TNLA to an instance of the NG-C interface.
    /// Returns a revision number.  Each change in the interface state results in a new
    /// number.
    // TODO: add parameters that actually define the association.
    fn associate_connection(&self) -> i32;
}
