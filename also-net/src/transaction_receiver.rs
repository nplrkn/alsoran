use super::SharedTransactions;
use super::{TnlaEvent, TnlaEventHandler};
use async_trait::async_trait;
use slog::{trace, warn, Logger};

#[derive(Clone)]
pub struct TransactionReceiver<R, M>
where
    R: TnlaEventHandler<M>,
    M: Clone + Send + Sync + 'static,
{
    pending_requests: SharedTransactions<M>,
    receiver: R,
}

impl<R, M> TransactionReceiver<R, M>
where
    R: TnlaEventHandler<M>,
    M: Clone + Send + Sync + 'static,
{
    pub fn new(receiver: R, transactions: SharedTransactions<M>) -> Self {
        TransactionReceiver {
            pending_requests: transactions,
            receiver,
        }
    }
}

#[async_trait]
impl<R, M> TnlaEventHandler<M> for TransactionReceiver<R, M>
where
    R: TnlaEventHandler<M>,
    M: Clone + Send + Sync + 'static,
{
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        self.receiver.handle_event(event, tnla_id, logger).await
    }

    async fn handle_message(&self, message: M, tnla_id: u32, logger: &Logger) {
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
