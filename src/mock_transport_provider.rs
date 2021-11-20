use super::transport_provider::{Handler, Message, TransportProvider};
use async_channel::{Receiver, Sender};
use async_trait::async_trait;
use slog::info;
use slog::Logger;

/// MockServerTransportProvider
/// Provides a message passing mechanism for use by test scripts.
/// When the business logic calls non_ue_associated_message() or ue_associated_message(),
/// the message is passed through to the receive channel.
#[derive(Debug, Clone)]
pub struct MockServerTransportProvider {
    server_sender: Sender<Message>,
    server_receiver: Receiver<Message>,
}

impl MockServerTransportProvider {
    /// Create a mock transport provider.
    pub fn new() -> (
        MockServerTransportProvider,
        Sender<Vec<u8>>,
        Receiver<Vec<u8>>,
    ) {
        let (server_sender, client_receiver) = async_channel::unbounded();
        let (client_sender, server_receiver) = async_channel::unbounded();

        (
            MockServerTransportProvider {
                server_sender,
                server_receiver,
            },
            client_sender,
            client_receiver,
        )
    }
}

#[async_trait]
impl TransportProvider for MockServerTransportProvider {
    async fn send_message(&self, message: Message, logger: &Logger) -> Result<(), String> {
        info!(
            logger,
            "MockServerTransportProvider send message {:?}", message
        );
        self.server_sender.send(message).await.unwrap();
        Ok(())
    }
    async fn recv_message(&self) -> Option<Message> {
        let message = self.server_receiver.recv().await;
        message.ok()
    }
    async fn start_receiving<R: Handler>(&self, receiver: R, logger: &Logger) {
        let server_receiver = self.server_receiver.clone();
        let logger = logger.clone();
        async_std::task::spawn(async move {
            while let Ok(message) = server_receiver.recv().await {
                info!(
                    logger,
                    "MockServerTransportProvider received {:?}, forward to handler", message
                );
                receiver.recv_non_ue_associated(message, &logger).await;
            }
        });
    }
}
