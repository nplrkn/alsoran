//! e1ap - E1AP entry points into the GNB-CU

use crate::workflows::Workflow;

use crate::GnbCuUp;
use async_trait::async_trait;
use e1ap::*;
use net::{EventHandler, RequestError, RequestProvider, ResponseAction, TnlaEvent};
use slog::{debug, Logger};

#[derive(Clone)]
pub struct E1apHandler<G: GnbCuUp> {
    gnb_cu_up: G,
}

impl<G: GnbCuUp> E1apHandler<G> {
    pub fn new_e1ap_application(gnb_cu_up: G) -> E1apUp<E1apHandler<G>> {
        E1apUp::new(E1apHandler { gnb_cu_up })
    }
}

#[async_trait]
impl<G: GnbCuUp> EventHandler for E1apHandler<G> {
    async fn handle_event(&self, event: TnlaEvent, _tnla_id: u32, logger: &Logger) {
        debug!(logger, "TNLA event {:?}", event)
    }
}

#[async_trait]
impl<G: GnbCuUp> RequestProvider<BearerContextSetupProcedure> for E1apHandler<G> {
    async fn request(
        &self,
        r: BearerContextSetupRequest,
        logger: &Logger,
    ) -> Result<ResponseAction<BearerContextSetupResponse>, RequestError<BearerContextSetupFailure>>
    {
        Workflow::new(&self.gnb_cu_up, logger)
            .bearer_context_setup(&r)
            .await
    }
}
