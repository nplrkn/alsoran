use super::Gnbcu;
use slog::Logger;

mod build_f1ap;
mod build_rrc;
mod downlink_nas;
mod initial_access;
mod initial_context_setup;
mod ng_setup;
mod pdu_session_resource_setup;
mod uplink_nas;

pub struct Workflow<'a, G: Gnbcu> {
    gnbcu: &'a G,
    logger: &'a Logger,
}

impl<'a, G: Gnbcu> Workflow<'a, G> {
    pub fn new(gnbcu: &'a G, logger: &'a Logger) -> Self {
        Workflow { gnbcu, logger }
    }
}

impl<'a, G: Gnbcu> std::ops::Deref for Workflow<'a, G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        &self.gnbcu
    }
}
