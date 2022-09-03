use super::Gnbcu;
use async_trait::async_trait;
use e1ap::*;
use net::{EventHandler, RequestError, RequestProvider, TnlaEvent};
use slog::{debug, info, warn, Logger};

#[derive(Clone)]
pub struct E1apHandler<G: Gnbcu> {
    gnbcu: G,
}

impl<G: Gnbcu> E1apHandler<G> {
    pub fn new_e1ap_application(gnbcu: G) -> E1apCp<E1apHandler<G>> {
        E1apCp::new(E1apHandler { gnbcu: gnbcu })
    }
}

#[async_trait]
impl<G: Gnbcu> RequestProvider<GnbCuUpE1SetupProcedure> for E1apHandler<G> {
    async fn request(
        &self,
        r: GnbCuUpE1SetupRequest,
        logger: &Logger,
    ) -> Result<GnbCuUpE1SetupResponse, RequestError<GnbCuUpE1SetupFailure>> {
        debug!(logger, ">> GnbCuUpE1SetupRequest");
        info!(
            logger,
            "E1AP interface initialized with {:?}", r.gnb_cu_up_id
        );
        debug!(logger, "<< GnbCuUpE1SetupResponse");
        Ok(GnbCuUpE1SetupResponse {
            transaction_id: r.transaction_id,
            gnb_cu_cp_name: self.gnbcu.config().clone().name.map(|x| GnbCuCpName(x)),
            transport_layer_address_info: None,
            extended_gnb_cu_cp_name: None,
        })
    }
}

#[async_trait]
impl<G: Gnbcu> EventHandler for E1apHandler<G> {
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        match event {
            TnlaEvent::Established(addr) => {
                info!(logger, "E1AP TNLA {} established from {}", tnla_id, addr)
            }
            TnlaEvent::Terminated => warn!(logger, "E1AP TNLA {} closed", tnla_id),
        };
    }
}
