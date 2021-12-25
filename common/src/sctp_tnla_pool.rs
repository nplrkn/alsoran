use crate::sctp::SctpAssociation;
use crate::transport_provider::{Handler, Message, TransportProvider};
use anyhow::anyhow;
use anyhow::Result;
use async_std::sync::{Arc, Mutex};
use async_std::task::JoinHandle;
use async_trait::async_trait;
use futures::{pin_mut, FutureExt};
use slog::{info, trace, Logger};
use std::collections::HashMap;
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

    pub async fn add_and_handle_no_spawn<H: Handler>(
        &self,
        assoc_id: u32,
        assoc: Arc<SctpAssociation>,
        handler: H,
        stop_token: StopToken,
        logger: Logger,
    ) {
        trace!(logger, "Wait on lock to add assoc {:?} to pool", assoc_id);
        self.assocs.lock().await.insert(assoc_id, assoc.clone());

        trace!(logger, "Notify TNLA established");
        handler.tnla_established(assoc_id, &logger).await;

        trace!(logger, "Start TNLA event loop");
        let fused_stop_token = stop_token.fuse();
        pin_mut!(fused_stop_token);
        loop {
            let next = assoc.recv_msg().fuse();
            pin_mut!(next);
            futures::select! {
                message = next => match message {
                    Ok(message) => handler.recv_non_ue_associated(message, &logger).await,
                    Err(e) => {
                        info!(logger, "TNLA terminated - {:?}", e);
                        handler.tnla_terminated(assoc_id, &logger).await
                    }
                },
                () = fused_stop_token => break
            }
        }

        trace!(logger, "Wait on lock to remove assoc {:?}", assoc_id);
        self.assocs.lock().await.remove(&assoc_id);
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
        H: Handler,
    {
        async_std::task::spawn(async move {
            self.add_and_handle_no_spawn(assoc_id, assoc, handler, stop_token, logger)
                .await;
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
