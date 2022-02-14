use super::{SharedTransactions, TransportProvider};
use crate::TransactionMatchFn;
use anyhow::Result;
use async_channel;
use slog::Logger;

#[derive(Clone)]
pub struct TransactionSender<T, M>
where
    M: Clone + Send + Sync + 'static,
    T: TransportProvider<Pdu = M>,
{
    pending_requests: SharedTransactions<M>,
    pub transport_provider: T,
}

impl<T, M> TransactionSender<T, M>
where
    M: Clone + Send + Sync + 'static,
    T: TransportProvider<Pdu = M>,
{
    pub fn new(transport_provider: T, transactions: SharedTransactions<M>) -> Self {
        TransactionSender {
            pending_requests: transactions,
            transport_provider,
        }
    }

    pub async fn send_request(
        &self,
        pdu: M,
        match_fn: TransactionMatchFn<M>,
        logger: &Logger,
    ) -> Result<M> {
        let (sender, receiver) = async_channel::bounded::<M>(1);

        // TODO - timeout
        self.pending_requests
            .lock()
            .await
            .push((Box::new(match_fn), sender));
        self.transport_provider.send_pdu(pdu, logger).await?;
        Ok(receiver.recv().await?)
    }
}
