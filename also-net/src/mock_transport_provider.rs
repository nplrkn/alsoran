use super::transport_provider::{
    ClientTransportProvider, ServerTransportProvider, TransportProvider,
};
use crate::tnla_event_handler::{TnlaEvent, TnlaEventHandler};
use anyhow::Result;
use async_channel::{Receiver, Sender};
use async_std::task::JoinHandle;
use async_trait::async_trait;
use futures::stream::StreamExt;
use slog::{trace, Logger};
use std::fmt::Debug;
use std::net::SocketAddr;
use stop_token::StopToken;

/// MockTransportProvider
/// Provides a message passing mechanism for use by test scripts.
/// When the business logic calls non_ue_associated_message() or ue_associated_message(),
/// the message is passed through to the receive channel.
#[derive(Debug, Clone)]
pub struct MockTransportProvider<P: Send + Sync + Clone + 'static + Debug> {
    sender: Sender<P>,
    receiver: Receiver<P>,
}

impl<P: Send + Sync + Clone + 'static + Debug> MockTransportProvider<P> {
    /// Create a mock transport provider.
    pub fn new() -> (MockTransportProvider<P>, Sender<P>, Receiver<P>) {
        let (sender, their_receiver) = async_channel::unbounded();
        let (their_sender, receiver) = async_channel::unbounded();

        (
            MockTransportProvider { sender, receiver },
            their_sender,
            their_receiver,
        )
    }
}

#[async_trait]
impl<P: Send + Sync + Clone + 'static + Debug> TransportProvider for MockTransportProvider<P> {
    type Pdu = P;
    async fn send_pdu(&self, pdu: P, logger: &Logger) -> Result<()> {
        trace!(logger, "MockTransportProvider send message {:?}", pdu);
        self.sender.send(pdu).await.unwrap();
        Ok(())
    }
}

#[async_trait]
impl<P: Send + Sync + Clone + 'static + Debug> ClientTransportProvider
    for MockTransportProvider<P>
{
    type Pdu = P;
    async fn maintain_connection<H>(
        self,
        _connect_addr_string: String,
        handler: H,
        stop_token: StopToken,
        logger: Logger,
    ) -> Result<JoinHandle<()>>
    where
        H: TnlaEventHandler<MessageType = P>,
    {
        let receiver = self.receiver.clone();
        handler
            .handle_event(TnlaEvent::Established, 1, &logger)
            .await;
        let mut stream = receiver.take_until(stop_token);
        Ok(async_std::task::spawn(async move {
            while let Some(pdu) = stream.next().await {
                trace!(
                    logger,
                    "MockTransportProvider received {:?}, forward to handler",
                    pdu
                );
                handler.handle_message(pdu, 1, &logger).await;
            }
        }))
    }

    async fn remote_tnla_addresses(&self) -> Vec<SocketAddr> {
        unimplemented!()
    }
}

#[async_trait]
impl<P: Send + Sync + Clone + 'static + Debug> ServerTransportProvider
    for MockTransportProvider<P>
{
    type Pdu = P;
    async fn serve<H>(
        self,
        _listen_addr: String,
        _stop_token: StopToken,
        _handler: H,
        _logger: Logger,
    ) -> Result<JoinHandle<()>>
    where
        H: TnlaEventHandler<MessageType = P>,
    {
        unimplemented!()
    }
}
