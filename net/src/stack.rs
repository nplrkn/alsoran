// stack - transaction layer allowing workflow business logic to await a response to its ??AP requests

use crate::tnla_event_handler::TnlaEventHandler;
use crate::{
    Indication, IndicationHandler, Message, Procedure, RequestError, RequestMessageHandler,
    RequestProvider, SctpTransportProvider, ShutdownHandle, TnlaEvent, TransportProvider,
};
use anyhow::Result;
use async_channel::Sender;
use async_net::SocketAddr;
use async_std::sync::{Arc, Mutex};
use async_trait::async_trait;
use slog::{trace, warn, Logger};

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
        connect_address: &String,
        ppid: u32,
        application: A,
        logger: Logger,
    ) -> Result<()> {
        let receiver = StackReceiver {
            application,
            pending_requests: self.pending_requests.clone(),
        };
        self.transport_provider
            .clone()
            .connect(connect_address, ppid, receiver, logger)
            .await
    }

    pub async fn listen<A: Application>(
        &self,
        listen_address: String,
        ppid: u32,
        application: A,
        logger: Logger,
    ) -> Result<ShutdownHandle> {
        let receiver = StackReceiver {
            application,
            pending_requests: self.pending_requests.clone(),
        };

        self.transport_provider
            .clone()
            .serve(listen_address, ppid, receiver, logger)
            .await
    }

    pub async fn remote_tnla_addresses(&self) -> Vec<SocketAddr> {
        self.transport_provider.remote_tnla_addresses().await
    }
}

#[async_trait]
impl<P: Procedure> RequestProvider<P> for Stack {
    async fn request(
        &self,
        r: P::Request,
        logger: &Logger,
    ) -> Result<P::Success, RequestError<P::Failure>> {
        let bytes = P::encode_request(r)?;

        // Create a channel to receive the response.
        let (sender, receiver) = async_channel::bounded::<Vec<u8>>(1);
        let match_fn = |m: &Message| ((m[0] != 0) && (m[1] == P::CODE));
        self.pending_requests
            .lock()
            .await
            .push((Box::new(match_fn), sender));

        self.transport_provider.send_message(bytes, logger).await?;

        // TODO - timeout
        let msg = receiver.recv().await?;
        P::decode_response(&msg)
    }
}

#[async_trait]
impl<I: Indication> IndicationHandler<I> for Stack {
    async fn handle(&self, i: I::Request, logger: &Logger) {
        match I::encode_request(i) {
            Ok(m) => match self.transport_provider.send_message(m, logger).await {
                Ok(()) => (),
                Err(e) => warn!(logger, "Error sending indication - {:?}", e),
            },
            Err(e) => warn!(logger, "Error encoding indication - {:?}", e),
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

    async fn handle_message(
        &self,
        message: Message,
        _tnla_id: u32,
        logger: &Logger,
    ) -> Option<Message> {
        // TODO figure out if it is a response and warn / drop if there are no matches

        // If it matches a pending request, route it back over the response channel.

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
                None
            }
            _ => self.application.handle_request(&message, logger).await,
        }
    }
}
