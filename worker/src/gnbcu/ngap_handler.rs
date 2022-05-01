use super::Gnbcu;
use async_trait::async_trait;
use net::{EventHandler, RequestProvider, TnlaEvent};
use ngap::{NgSetupRequestProcedure, NgapGnb};
use slog::{trace, warn, Logger};

impl RequestProvider<NgSetupRequestProcedure> for Handler {}

pub fn new(gnbcu: Gnbcu) -> NgapGnb<Handler> {
    NgapGnb(Handler { gnbcu })
}
#[derive(Clone)]
pub struct Handler {
    pub gnbcu: Gnbcu,
}

#[async_trait]
impl EventHandler for Handler {
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        match event {
            TnlaEvent::Established => trace!(logger, "NGAP TNLA {} established", tnla_id),
            TnlaEvent::Terminated => warn!(logger, "NGAP TNLA {} closed", tnla_id),
        };
        self.gnbcu.connected_amf_change(logger).await;
    }
}
