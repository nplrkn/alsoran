//! sctp_tnla_pool - global connection pool enabling a suitable TNLA to be selected for an outgoing message

use crate::{
    tnla_event_handler::{TnlaEvent, TnlaEventHandler},
    transport_provider::{AssocId, Binding},
};
use anyhow::{anyhow, ensure, Result};
use async_std::sync::{Arc, Mutex};
use common::ShutdownHandle;
use futures::pin_mut;
use futures::stream::StreamExt;
use sctp::{Message, SctpAssociation};
use slog::{debug, warn, Logger};
use std::collections::HashMap;
use std::net::SocketAddr;
use stop_token::{StopSource, StopToken};
type SharedAssocHash = Arc<Mutex<Box<HashMap<AssocId, Arc<SctpAssociation>>>>>;

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

    /// Picks a new binding (association and in future stream ID).  
    /// To load balance among different associations, use a different seed.
    pub async fn new_ue_binding(&self, seed: u32) -> Result<Binding> {
        let assocs = self.assocs.lock().await;
        ensure!(assocs.len() > 0, "No associations up");
        let nth = seed as usize % assocs.len();
        Ok(Binding {
            assoc_id: *assocs.keys().nth(nth).unwrap(),
        })
    }

    pub async fn send_message(
        &self,
        message: Message,
        assoc_id: Option<u32>,
        logger: &Logger,
    ) -> Result<()> {
        let assocs = self.assocs.lock().await;
        if let Some((id, assoc)) = if let Some(assoc_id) = assoc_id {
            // Use the specified association
            assocs.get(&assoc_id).map(|x| (assoc_id, x))
        } else {
            // Use the first one
            assocs.iter().next().map(|(k, v)| (*k, v))
        } {
            debug!(logger, "Send message on assoc {}", id);
            Ok(assoc.send_msg(message).await?)
        } else {
            Err(anyhow!("No association found"))
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
        self.assocs.lock().await.insert(assoc_id, assoc.clone());
        let shutdown_handle = ShutdownHandle::new(
            async_std::task::spawn(async move {
                self_clone
                    .handle_assoc(assoc_id, assoc, handler, stop_token, logger)
                    .await;
            }),
            stop_source,
        );
        self.tasks.lock().await.push(shutdown_handle);
    }

    async fn handle_assoc<H>(
        &self,
        assoc_id: AssocId,
        assoc: Arc<SctpAssociation>,
        handler: H,
        stop_token: StopToken,
        logger: Logger,
    ) where
        H: TnlaEventHandler,
    {
        // Notify new association establishment.  Note that we do not spawn a separate task for this event.  This is
        // to guarantee that the establishment event is processed before we move onto delivering any packets to the handler.
        handler
            .handle_event(
                TnlaEvent::Established(assoc.remote_address),
                assoc_id,
                &logger,
            )
            .await;

        let message_stream = assoc.recv_msg_stream().take_until(stop_token);
        pin_mut!(message_stream);
        loop {
            match message_stream.next().await {
                // TODO - this does not implement graceful shutdown.  For one thing, the Terminated events below
                // may overtake message events.  For another, graceful shutdown implies that procedures currently underway
                // will allowed to complete sucessfully, which means we need to keep receiving on the association until
                // they are complete.  But above, we just cut off the stream as soon as the stop token fired.

                // Local shutdown
                None => {
                    spawn_handle_event(
                        handler.clone(),
                        TnlaEvent::Terminated,
                        assoc_id,
                        logger.clone(),
                    );
                    break;
                }
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
    debug!(logger, "Received message on assoc {}", assoc_id);
    if let Some(response) = handler.handle_message(message, assoc_id, &logger).await {
        if let Err(e) = assoc.send_msg(response).await {
            warn!(logger, "Failed to send response - {}", e)
        }
    }
}
