//! e1ap - E1AP entry points into the GNB-CU

use crate::workflows::Workflow;

use super::Gnbcu;
use async_trait::async_trait;
use e1ap::*;
use net::{EventHandler, RequestError, RequestProvider, TnlaEvent};
use slog::{info, Logger};

#[derive(Clone)]
pub struct E1apHandler<G: Gnbcu> {
    gnbcu: G,
}

impl<G: Gnbcu> E1apHandler<G> {
    pub fn new_e1ap_application(gnbcu: G) -> E1apCp<E1apHandler<G>> {
        E1apCp::new(E1apHandler { gnbcu })
    }
}

#[async_trait]
impl<G: Gnbcu> RequestProvider<GnbCuUpE1SetupProcedure> for E1apHandler<G> {
    async fn request(
        &self,
        r: GnbCuUpE1SetupRequest,
        logger: &Logger,
    ) -> Result<GnbCuUpE1SetupResponse, RequestError<GnbCuUpE1SetupFailure>> {
        Workflow::new(&self.gnbcu, logger).e1_setup(r).await
    }
}

#[async_trait]
impl<G: Gnbcu> EventHandler for E1apHandler<G> {
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        match event {
            TnlaEvent::Established(addr) => {
                info!(logger, "E1AP TNLA {} established from {}", tnla_id, addr)
            }
            TnlaEvent::Terminated => info!(logger, "E1AP TNLA {} closed", tnla_id),
        };
    }
}
