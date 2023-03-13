use crate::GnbCuUp;
use slog::{debug, Logger};
mod bearer_context_setup;

pub struct Workflow<'a, G: GnbCuUp> {
    gnb_cu_up: &'a G,
    logger: &'a Logger,
}

impl<'a, G: GnbCuUp> Workflow<'a, G> {
    pub fn new(gnb_cu_up: &'a G, logger: &'a Logger) -> Self {
        Workflow { gnb_cu_up, logger }
    }
    pub fn log_message(&self, s: &str) {
        debug!(self.logger, "{}", s)
    }
}

impl<'a, G: GnbCuUp> std::ops::Deref for Workflow<'a, G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        self.gnb_cu_up
    }
}
