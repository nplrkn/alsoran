use super::SharedTransactions;
use super::{TnlaEvent, TnlaEventHandler};
use async_trait::async_trait;
use sctp::Message;
use slog::{trace, warn, Logger};

#[derive(Clone)]
pub struct TransactionReceiver<R>
where
    R: TnlaEventHandler,
{
    // TODO - we could remove the unneeded clone of this by creating a trait like HasSharedTransactions<M> on <R>
    pending_requests: SharedTransactions,
    receiver: R,
}

impl<R> TransactionReceiver<R>
where
    R: TnlaEventHandler,
{
    pub fn new(receiver: R, transactions: SharedTransactions) -> Self {
        TransactionReceiver {
            pending_requests: transactions,
            receiver,
        }
    }
}

// TODO: Right now, this filters out the responses and pass on the requests and events.
// In future it should spawn requests surely?
#[async_trait]
impl<R> TnlaEventHandler for TransactionReceiver<R>
where
    R: TnlaEventHandler,
{
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        self.receiver.handle_event(event, tnla_id, logger).await
    }

    async fn handle_message(&self, message: Message, tnla_id: u32, logger: &Logger) {
        // TODO figure out if it is a response and warn / drop if there are no matches

        // If it matches a pending request, route it back over the response channel.

        // TODO switch to read write lock
        let position = self
            .pending_requests
            .lock()
            .await
            .iter()
            .position(|(matches, _)| matches(&message));

        match position {
            Some(index) => {
                trace!(logger, "Matched the transaction at position {}", index);
                let (_, response_channel) = self.pending_requests.lock().await.swap_remove(index);
                response_channel
                    .send(message)
                    .await
                    .unwrap_or_else(|_| warn!(logger, "Internal response channel down"));
            }
            _ => self.receiver.handle_message(message, tnla_id, logger).await,
        };
    }
}
