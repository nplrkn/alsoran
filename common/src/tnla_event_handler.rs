//use crate::ngap::NgapPdu;
use async_trait::async_trait;
use sctp::Message;
use serde::de::DeserializeOwned;
use serde_json;
use slog::{trace, Logger};
use std::fmt::Debug;

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

pub trait PduTypeBounds: DeserializeOwned + Send + Sync + Clone + 'static + Debug {}

// Wrap a handler in a JSON decoder to decode from JSON.
#[derive(Clone)]
pub struct JsonDecoder<T, P>(pub T)
where
    T: TnlaEventHandler<MessageType = P>,
    P: DeserializeOwned + Send + Sync + Clone + 'static + Debug;

#[async_trait]
impl<T, P> TnlaEventHandler for JsonDecoder<T, P>
where
    P: DeserializeOwned + Send + Sync + Clone + Debug,
    T: TnlaEventHandler<MessageType = P>,
{
    type MessageType = Message;

    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        self.0.handle_event(event, tnla_id, logger).await
    }

    // TODO indicate whether it is UE or non UE associated?
    async fn handle_message(&self, message: Message, tnla_id: u32, logger: &Logger) {
        trace!(logger, "JSON decode of message {:?}", message);
        let converted: P = serde_json::from_slice(&message).unwrap();
        self.0.handle_message(converted, tnla_id, logger).await
    }
}
