use crate::procedures::{self, GnbcuOps};
use anyhow::Result;
use async_trait::async_trait;
use net::{EventHandler, IndicationHandler, RequestError, RequestProvider, TnlaEvent};
use ngap::*;
use slog::{debug, info, warn, Logger};

impl<G: GnbcuOps> RequestProvider<NgSetupProcedure> for NgapHandler<G> {}

#[derive(Clone)]
pub struct NgapHandler<G> {
    gnbcu: G,
}

impl<G: GnbcuOps> NgapHandler<G> {
    pub fn new_ngap_application(gnbcu: G) -> NgapGnb<NgapHandler<G>> {
        NgapGnb::new(NgapHandler { gnbcu })
    }
}
#[async_trait]
impl<G: GnbcuOps> EventHandler for NgapHandler<G> {
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        match event {
            TnlaEvent::Established(addr) => {
                info!(logger, "NGAP TNLA {} established to {}", tnla_id, addr);
                crate::procedures::ng_setup(&self.gnbcu, logger).await;
            }
            TnlaEvent::Terminated => warn!(logger, "NGAP TNLA {} closed", tnla_id),
        };
        // TODO
    }
}

#[async_trait]
impl<G: GnbcuOps> IndicationHandler<DownlinkNasTransportProcedure> for NgapHandler<G> {
    async fn handle(&self, i: DownlinkNasTransport, logger: &Logger) {
        crate::procedures::downlink_nas(&self.gnbcu, i, logger).await;
    }
}

#[async_trait]
impl<G: GnbcuOps> RequestProvider<InitialContextSetupProcedure> for NgapHandler<G> {
    async fn request(
        &self,
        r: InitialContextSetupRequest,
        logger: &Logger,
    ) -> Result<InitialContextSetupResponse, RequestError<InitialContextSetupFailure>> {
        debug!(logger, "Initial Context Setup Procedure");
        procedures::initial_context_setup(&self.gnbcu, &r, logger)
            .await
            .map_err(|cause| {
                RequestError::UnsuccessfulOutcome(InitialContextSetupFailure {
                    amf_ue_ngap_id: r.amf_ue_ngap_id,
                    ran_ue_ngap_id: r.ran_ue_ngap_id,
                    pdu_session_resource_failed_to_setup_list_cxt_fail: None,
                    cause,
                    criticality_diagnostics: None,
                })
            })
    }
}

#[async_trait]
impl<G: GnbcuOps> IndicationHandler<AmfStatusIndicationProcedure> for NgapHandler<G> {
    async fn handle(&self, i: AmfStatusIndication, logger: &Logger) {
        debug!(logger, "<< Amf Status Indication");
        for guami_item in i.unavailable_guami_list.0 {
            info!(logger, "GUAMI {} now unavailable", guami_item.guami);
        }
    }
}
