pub use crate::Config;
use crate::{
    datastore::{UeState, UeStateStore},
    rrc_transaction::RrcTransaction,
};
use async_channel::Sender;
use async_trait::async_trait;
use net::{Indication, Procedure, RequestError};
use rrc::UlDcchMessage;
use slog::Logger;

/// GnbcuT - trait representing a Gnbcu instance on top of which Gnbcu business logic can be implemented.
#[async_trait]
pub trait GnbcuT: Send + Sync + Clone + 'static + UeStateStore {
    fn config(&self) -> &Config;

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

    /// Start a new RRC transaction.
    async fn new_rrc_transaction(&self, ue: &UeState) -> RrcTransaction;

    /// Determine if this is a response to a local pending RRC transaction.
    async fn match_rrc_transaction(&self, ue_id: u32) -> Option<Sender<UlDcchMessage>>;

    async fn send_rrc_to_ue(
        &self,
        ue: &UeState,
        rrc_container: f1ap::RrcContainer,
        logger: &Logger,
    );
}
