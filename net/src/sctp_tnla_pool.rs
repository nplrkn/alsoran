use crate::tnla_event_handler::{TnlaEvent, TnlaEventHandler};
use anyhow::anyhow;
use anyhow::Result;
use async_std::sync::{Arc, Mutex};
use async_std::task::JoinHandle;
use futures::pin_mut;
use futures::stream::StreamExt;
use sctp::{Message, SctpAssociation};
use slog::{trace, Logger};
use std::collections::HashMap;
use std::net::SocketAddr;
use stop_token::StopToken;

type TnlaId = u32;
type SharedAssocHash = Arc<Mutex<Box<HashMap<TnlaId, Arc<SctpAssociation>>>>>;

#[derive(Debug, Clone)]
pub struct SctpTnlaPool {
    assocs: SharedAssocHash,
}

impl SctpTnlaPool {
    pub fn new() -> SctpTnlaPool {
        let assocs = Arc::new(Mutex::new(Box::new(HashMap::new())));
        SctpTnlaPool { assocs }
    }

    pub async fn remote_addresses(&self) -> Vec<SocketAddr> {
        self.assocs
            .lock()
            .await
            .values()
            .map(|assoc| assoc.remote_address)
            .collect()
    }

    pub async fn add_and_handle_no_spawn<H>(
        &self,
        assoc_id: u32,
        assoc: Arc<SctpAssociation>,
        handler: H,
        stop_token: StopToken,
        logger: Logger,
    ) where
        H: TnlaEventHandler<Message>,
    {
        trace!(logger, "Wait on lock to add assoc {:?} to pool", assoc_id);
        self.assocs.lock().await.insert(assoc_id, assoc.clone());

        trace!(logger, "Notify TNLA established");
        handler
            .handle_event(TnlaEvent::Established, assoc_id, &logger)
            .await;

        trace!(logger, "Start TNLA event loop");
        let message_stream = assoc.recv_msg_stream().take_until(stop_token);
        pin_mut!(message_stream);
        while let Some(Ok(message)) = message_stream.next().await {
            handler.handle_message(message, assoc_id, &logger).await;
        }
        handler
            .handle_event(TnlaEvent::Terminated, assoc_id, &logger)
            .await;

        trace!(logger, "Wait on lock to remove assoc {:?}", assoc_id);
        self.assocs.lock().await.remove(&assoc_id);
    }

    pub async fn send_message(&self, message: Message, _logger: &Logger) -> Result<()> {
        if let Some(assoc) = self.assocs.lock().await.values().next() {
            Ok(assoc.send_msg(message).await?)
        } else {
            Err(anyhow!("No association up"))
        }
    }

    pub async fn add_and_handle<H>(
        self,
        assoc_id: u32,
        assoc: Arc<SctpAssociation>,
        handler: H,
        stop_token: StopToken,
        logger: Logger,
    ) -> JoinHandle<()>
    where
        H: TnlaEventHandler<Message>,
    {
        async_std::task::spawn(async move {
            self.add_and_handle_no_spawn(assoc_id, assoc, handler, stop_token, logger)
                .await;
        })
    }
}
