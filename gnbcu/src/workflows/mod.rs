use super::Gnbcu;
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
mod initial_access;
mod initial_context_setup;
mod ng_setup;
mod pdu_session_resource_setup;
mod ran_configuration_update;
mod uplink_nas;

pub struct Workflow<'a, G: Gnbcu> {
    gnbcu: &'a G,
    logger: &'a Logger,
}

impl<'a, G: Gnbcu> Workflow<'a, G> {
    pub fn new(gnbcu: &'a G, logger: &'a Logger) -> Self {
        Workflow { gnbcu, logger }
    }
    pub fn log_message(&self, s: &str) {
        debug!(self.logger, "{}", s)
    }
}

impl<'a, G: Gnbcu> std::ops::Deref for Workflow<'a, G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        self.gnbcu
    }
}
