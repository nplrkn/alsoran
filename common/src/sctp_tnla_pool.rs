use crate::sctp::SctpAssociation;
use crate::transport_provider::{Handler, Message, TransportProvider};
use anyhow::anyhow;
use anyhow::Result;
use async_std::sync::{Arc, Mutex};
use async_std::task::JoinHandle;
use async_trait::async_trait;
use slog::{info, trace, Logger};
use std::collections::HashMap;

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

    pub async fn add_and_handle<H>(
        &self,
        assoc_id: u32,
        assoc: Arc<SctpAssociation>,
        handler: H,
        logger: Logger,
    ) -> JoinHandle<()>
    where
        H: Handler,
    {
        trace!(logger, "Wait on lock to add assoc {:?} to pool", assoc_id);
        self.assocs.lock().await.insert(assoc_id, assoc.clone());

        handler.tnla_established(assoc_id, &logger).await;

        let assocs = self.assocs.clone();
        async_std::task::spawn(async move {
            trace!(logger, "Started handler");
            while let Ok(message) = assoc.recv_msg().await {
                handler.recv_non_ue_associated(message, &logger).await;
            }
            info!(logger, "Connection terminated");

            trace!(logger, "Wait on lock to remove assoc {:?}", assoc_id);
            assocs.lock().await.remove(&assoc_id);
        })
    }
}

#[async_trait]
impl TransportProvider for SctpTnlaPool {
    async fn send_message(&self, message: Message, _logger: &Logger) -> Result<()> {
        if let Some(assoc) = self.assocs.lock().await.values().next() {
            Ok(assoc.send_msg(message).await?)
        } else {
            Err(anyhow!("No association up"))
        }
    }
}
