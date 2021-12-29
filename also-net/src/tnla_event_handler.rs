//use crate::ngap::NgapPdu;
use async_trait::async_trait;
use slog::Logger;

#[async_trait]
pub trait TnlaEventHandler: 'static + Send + Sync + Clone {
    type MessageType;
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger);

    // TODO indicate whether it is UE or non UE associated?
    async fn handle_message(&self, message: Self::MessageType, tnla_id: u32, logger: &Logger);
}

pub enum TnlaEvent {
    Established,
    Terminated,
}
