use crate::{Codec, TnlaEvent, TnlaEventHandler};
use async_trait::async_trait;
use sctp::Message;
use slog::{trace, warn, Logger};
use std::fmt::Debug;

#[derive(Clone)]
pub struct Wrapper<T, P, C>
where
    T: TnlaEventHandler<P>,
    P: Send + Sync + Clone + 'static + Debug,
    C: Codec<Pdu = P> + Clone + Send + Sync + 'static,
{
    pub handler: T,
    pub codec: C,
}

#[async_trait]
impl<T, P, C> TnlaEventHandler<Message> for Wrapper<T, P, C>
where
    T: TnlaEventHandler<P>,
    P: Send + Sync + Clone + 'static + Debug,
    C: Codec<Pdu = P> + Clone + Send + Sync + 'static,
{
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        self.handler.handle_event(event, tnla_id, logger).await
    }

    async fn handle_message(&self, message: Message, tnla_id: u32, logger: &Logger) {
        trace!(logger, "Got bytes {:?}", message);
        match self.codec.from_wire(message) {
            Ok(converted) => {
                trace!(logger, "Successful decode");
                self.handler
                    .handle_message(converted, tnla_id, logger)
                    .await
            }
            Err(e) => warn!(logger, "Decode error {:?}", e),
        }
    }
}
