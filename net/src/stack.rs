// NGAP or F1AP stack

use crate::tnla_event_handler::TnlaEventHandler;
use crate::{
    AperCodec, Message, Procedure, RequestError, RequestMessageHandler, RequestProvider,
    SctpTransportProvider, TnlaEvent, TransportProvider,
};
use anyhow::Result;
use async_channel::Sender;
use async_net::SocketAddr;
use async_std::sync::{Arc, Mutex};
use async_std::task::JoinHandle;
use async_trait::async_trait;
use slog::{trace, warn, Logger};
use stop_token::StopSource;

type TransactionMatchFn = Box<dyn Fn(&Message) -> bool + Send + Sync>;
type SharedTransactions = Arc<Mutex<Box<Vec<(TransactionMatchFn, Sender<Message>)>>>>;

#[derive(Clone)]
pub struct Stack {
    pending_requests: SharedTransactions,
    transport_provider: SctpTransportProvider,
}

#[async_trait]
pub trait EventHandler: Clone + Send + Sync + 'static {
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger);
}

pub trait Application: EventHandler + RequestMessageHandler {}

impl Stack {
    pub fn new(transport_provider: SctpTransportProvider) -> Self {
        Self {
            transport_provider,
            pending_requests: Arc::new(Mutex::new(Box::new(Vec::new()))),
        }
    }

    pub async fn connect<A: Application>(
        &self,
        connect_address: String,
        application: A,
        logger: Logger,
    ) -> Result<TransportTasks> {
        let receiver = StackReceiver {
            application,
            pending_requests: self.pending_requests.clone(),
        };
        let stop_source = StopSource::new();

        let handle = self
            .transport_provider
            .clone()
            .maintain_connection(connect_address, stop_source.token(), receiver, logger)
            .await?;
        Ok(TransportTasks {
            handle,
            stop_source,
        })
    }

    pub async fn listen<A: Application>(
        &self,
        listen_address: String,
        application: A,
        logger: Logger,
    ) -> Result<TransportTasks> {
        let receiver = StackReceiver {
            application,
            pending_requests: self.pending_requests.clone(),
        };
        let stop_source = StopSource::new();

        let handle = self
            .transport_provider
            .clone()
            .serve(listen_address, stop_source.token(), receiver, logger)
            .await?;
        Ok(TransportTasks {
            handle,
            stop_source,
        })
    }

    pub async fn remote_tnla_addresses(&self) -> Vec<SocketAddr> {
        self.transport_provider.remote_tnla_addresses().await
    }
}

pub struct TransportTasks {
    handle: JoinHandle<()>,
    stop_source: StopSource,
}

impl TransportTasks {
    pub async fn graceful_shutdown(self) {
        drop(self.stop_source);
        self.handle.await
    }
}

#[async_trait]
impl<P: Procedure> RequestProvider<P> for Stack {
    async fn request(
        &self,
        r: P::Request,
        logger: &Logger,
    ) -> Result<P::Success, RequestError<P::Failure>> {
        let bytes = r.into_bytes()?;

        // Create a channel to receive the response.
        let (sender, receiver) = async_channel::bounded::<Vec<u8>>(1);
        let match_fn = |m: &Message| ((m[0] != 0) && (m[1] == P::CODE));
        self.pending_requests
            .lock()
            .await
            .push((Box::new(match_fn), sender));

        self.transport_provider.send_message(bytes, logger).await?;
        let msg = receiver.recv().await?;

        // TODO: encapsulate this in a trait - either the Procedure trait or a new trait similar to
        // into_pdu_bytes().
        if msg[0] == 1 {
            Ok(P::Success::from_bytes(&msg)?)
        } else {
            Ok(P::Success::from_bytes(&msg)?) // TODO unsuccess
        }
    }
}

#[derive(Clone)]
struct StackReceiver<A: Application> {
    application: A,
    pending_requests: SharedTransactions,
}

#[async_trait]
impl<A: Application> TnlaEventHandler for StackReceiver<A> {
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        self.application.handle_event(event, tnla_id, logger).await
    }

    async fn handle_message(&self, message: Message, _tnla_id: u32, logger: &Logger) {
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
                // TODO
            }
            _ => {
                self.application
                    .handle_request(&message, logger)
                    .await
                    .unwrap(); // TODO
            }
        };
    }
}
