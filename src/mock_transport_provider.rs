use super::transport_provider::{TransportMessageReceiver, TransportProvider};
use async_channel::{Receiver, Sender};
use async_std::stream::StreamExt;
use async_std::sync::Arc;
use async_trait::async_trait;
use std::marker::Sync;

/// MockServerTransportProvider
/// Provides a message passing mechanism for use by test scripts.
/// When the business logic calls non_ue_associated_message() or ue_associated_message(),
/// the message is passed through to the receive channel.
#[derive(Debug, Clone)]
pub struct MockServerTransportProvider<R: TransportMessageReceiver + Sync> {
    receive_handler: Option<R>,
    server_receiver: Receiver<Vec<u8>>,
}

impl<R: TransportMessageReceiver + Sync> MockServerTransportProvider<R> {
    /// Create a mock transport provider.
    pub fn new() -> (
        MockServerTransportProvider<R>,
        Sender<Vec<u8>>,
        Receiver<Vec<u8>>,
    ) {
        let (server_sender, client_receiver) = async_channel::unbounded();
        let (client_sender, server_receiver) = async_channel::unbounded();

        (
            MockServerTransportProvider {
                receive_handler: None,
                server_receiver,
            },
            client_sender,
            client_receiver,
        )
    }
}

#[async_trait]
impl<R: 'static + TransportMessageReceiver + Sync> TransportProvider
    for MockServerTransportProvider<R>
{
    type Receiver = R;

    async fn set_receiver(&mut self, receiver: R) {
        let mut receive_stream = self.server_receiver.clone();

        // When we receive a message, call the callback
        async_std::task::spawn(async move {
            while let Some(message) = receive_stream.next().await {
                receiver.recv_non_ue_associated_message(message).await;
            }
        });
    }

    async fn send_non_ue_associated_message(&self, buf: &[u8]) -> Result<usize, String> {
        unimplemented!();
    }
}

#[async_trait]
impl<R: TransportMessageReceiver + Sync> TransportMessageReceiver
    for MockServerTransportProvider<R>
{
    async fn recv_non_ue_associated_message(&self, buf: Vec<u8>) {
        unimplemented!();
    }
}
