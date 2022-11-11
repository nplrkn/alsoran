//! sctp_tnla_pool - global connection pool enabling a suitable TNLA to be selected for an outgoing message

use crate::tnla_event_handler::{TnlaEvent, TnlaEventHandler};
use anyhow::{anyhow, Result};
use async_std::sync::{Arc, Mutex};
use common::ShutdownHandle;
use futures::pin_mut;
use futures::stream::StreamExt;
use sctp::{Message, SctpAssociation};
use slog::{trace, warn, Logger};
use std::collections::HashMap;
use std::net::SocketAddr;
use stop_token::{StopSource, StopToken};

type TnlaId = u32;
type SharedAssocHash = Arc<Mutex<Box<HashMap<TnlaId, Arc<SctpAssociation>>>>>;

#[derive(Clone)]
pub struct SctpTnlaPool {
    assocs: SharedAssocHash,
    tasks: Arc<Mutex<Vec<ShutdownHandle>>>,
}

impl SctpTnlaPool {
    pub fn new() -> SctpTnlaPool {
        SctpTnlaPool {
            assocs: Arc::new(Mutex::new(Box::new(HashMap::new()))),
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn graceful_shutdown(self) {
        for task in self.tasks.lock().await.drain(..) {
            task.graceful_shutdown().await;
        }
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
        H: TnlaEventHandler,
    {
        self.assocs.lock().await.insert(assoc_id, assoc.clone());

        trace!(logger, "Notify TNLA established");
        spawn_handle_event(
            handler.clone(),
            TnlaEvent::Established(assoc.remote_address),
            assoc_id,
            logger.clone(),
        );

        trace!(logger, "Start TNLA event loop");
        let message_stream = assoc.recv_msg_stream().take_until(stop_token);
        pin_mut!(message_stream);
        loop {
            match message_stream.next().await {
                // Graceful shutdown
                None => break,
                // Remote end terminated connection
                Some(Err(_)) => {
                    spawn_handle_event(
                        handler.clone(),
                        TnlaEvent::Terminated,
                        assoc_id,
                        logger.clone(),
                    );
                    break;
                }
                // Received a message
                Some(Ok(message)) => {
                    async_std::task::spawn(handle_message(
                        handler.clone(),
                        message,
                        assoc.clone(),
                        assoc_id,
                        logger.clone(),
                    ));
                }
            }
        }

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
        &self,
        assoc_id: u32,
        assoc: Arc<SctpAssociation>,
        handler: H,
        logger: Logger,
    ) where
        H: TnlaEventHandler,
    {
        let stop_source = StopSource::new();
        let stop_token = stop_source.token();
        let self_clone = self.clone();
        let shutdown_handle = ShutdownHandle::new(
            async_std::task::spawn(async move {
                self_clone
                    .add_and_handle_no_spawn(assoc_id, assoc, handler, stop_token, logger)
                    .await;
            }),
            stop_source,
        );
        self.tasks.lock().await.push(shutdown_handle);
    }
}

fn spawn_handle_event<H: TnlaEventHandler>(
    handler: H,
    event: TnlaEvent,
    tnla_id: u32,
    logger: Logger,
) {
    async_std::task::spawn(async move { handler.handle_event(event, tnla_id, &logger).await });
}

async fn handle_message<H: TnlaEventHandler>(
    handler: H,
    message: Vec<u8>,
    assoc: Arc<SctpAssociation>,
    assoc_id: u32,
    logger: Logger,
) {
    if let Some(response) = handler.handle_message(message, assoc_id, &logger).await {
        if let Err(e) = assoc.send_msg(response).await {
            warn!(logger, "Failed to send response - {}", e)
        }
    }
}
