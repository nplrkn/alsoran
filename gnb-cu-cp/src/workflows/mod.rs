use super::GnbCuCp;
use slog::{debug, Logger};

mod amf_status_indication;
mod build_f1ap;
mod build_ngap;
mod build_rrc;
mod downlink_nas;
mod e1_setup;
mod f1_setup;
mod gnb_cu_configuration_update;
mod gnb_cu_cp_configuration_update;
mod gnb_du_configuration_update;
mod initial_access;
mod initial_context_setup;
mod ng_setup;
mod pdu_session_resource_setup;
mod ran_configuration_update;
mod uplink_nas;

pub struct Workflow<'a, G: GnbCuCp> {
    gnb_cu_cp: &'a G,
    logger: &'a Logger,
}

impl<'a, G: GnbCuCp> Workflow<'a, G> {
    pub fn new(gnb_cu_cp: &'a G, logger: &'a Logger) -> Self {
        Workflow { gnb_cu_cp, logger }
    }
    pub fn log_message(&self, s: &str) {
        debug!(self.logger, "{}", s)
    }
}

impl<'a, G: GnbCuCp> std::ops::Deref for Workflow<'a, G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        self.gnb_cu_cp
    }
}
