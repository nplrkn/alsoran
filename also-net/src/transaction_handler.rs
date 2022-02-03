use super::TransportProvider;
use super::{TnlaEvent, TnlaEventHandler};
use anyhow::Result;
use async_std::sync::{Arc, Mutex};
use async_trait::async_trait;
use futures::channel::oneshot::{self, Receiver, Sender};
use slog::{warn, Logger};
use std::collections::HashMap;

type SharedTransactionHash<M> = Arc<Mutex<Box<HashMap<u8, Sender<M>>>>>;

pub trait HasTransactionId {
    fn request_transaction_id(&self) -> u8;
    fn response_transaction_id(&self) -> Option<u8>;
}

#[derive(Clone)]
pub struct TransactionHandler<H, T, M>
where
    H: TnlaEventHandler<M>,
    M: Clone + Send + Sync + 'static + HasTransactionId,
    T: TransportProvider<Pdu = M>,
{
    pending_requests: SharedTransactionHash<M>,
    transport_provider: T,
    handler: H,
}

impl<H, T, M> TransactionHandler<H, T, M>
where
    H: TnlaEventHandler<M>,
    M: Clone + Send + Sync + 'static + HasTransactionId,
    T: TransportProvider<Pdu = M>,
{
    pub fn new(transport_provider: T, handler: H) -> Self {
        TransactionHandler {
            pending_requests: Arc::new(Mutex::new(Box::new(HashMap::new()))),
            transport_provider,
            handler,
        }
    }

    pub async fn send_request(&self, pdu: M, logger: &Logger) -> Result<Receiver<M>> {
        let (sender, receiver) = oneshot::channel::<M>();
        let transaction_id = pdu.request_transaction_id();

        // TODO - timeout
        self.pending_requests
            .lock()
            .await
            .insert(transaction_id, sender);
        self.transport_provider.send_pdu(pdu, logger).await?;

        Ok(receiver)
    }
}

#[async_trait]
impl<H, T, M> TnlaEventHandler<M> for TransactionHandler<H, T, M>
where
    H: TnlaEventHandler<M>,
    M: Clone + Send + Sync + 'static + HasTransactionId,
    T: TransportProvider<Pdu = M>,
{
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        self.handler.handle_event(event, tnla_id, logger).await
    }

    async fn handle_message(&self, message: M, tnla_id: u32, logger: &Logger) {
        match message.response_transaction_id() {
            None => self.handler.handle_message(message, tnla_id, logger).await,
            Some(transaction_id) => {
                match self.pending_requests.lock().await.remove(&transaction_id) {
                    None => warn!(
                        logger,
                        "Received an unexpected response with transaction ID {}", transaction_id
                    ),
                    Some(oneshot) => oneshot.send(message).unwrap_or_else(|_| {
                        warn!(
                            logger,
                            "Internal response channel down for transaction ID {}", transaction_id
                        )
                    }),
                }
            }
        };
    }
}
