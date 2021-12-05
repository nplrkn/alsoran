use super::transport_provider::{ClientTransportProvider, Handler, Message, TransportProvider};
use anyhow::Result;
use async_channel::{Receiver, Sender};
use async_trait::async_trait;
use slog::info;
use slog::Logger;

/// MockTransportProvider
/// Provides a message passing mechanism for use by test scripts.
/// When the business logic calls non_ue_associated_message() or ue_associated_message(),
/// the message is passed through to the receive channel.
#[derive(Debug, Clone)]
pub struct MockTransportProvider {
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl MockTransportProvider {
    /// Create a mock transport provider.
    pub fn new() -> (MockTransportProvider, Sender<Vec<u8>>, Receiver<Vec<u8>>) {
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
impl TransportProvider for MockTransportProvider {
    async fn send_message(&self, message: Message, logger: &Logger) -> Result<()> {
        info!(logger, "MockTransportProvider send message {:?}", message);
        self.sender.send(message).await.unwrap();
        Ok(())
    }
    async fn start_receiving<R: Handler>(&self, handler: R, logger: &Logger) {
        let my_receiver = self.receiver.clone();
        let logger = logger.clone();
        async_std::task::spawn(async move {
            while let Ok(message) = my_receiver.recv().await {
                info!(
                    logger,
                    "MockTransportProvider received {:?}, forward to handler", message
                );
                handler.recv_non_ue_associated(message, &logger).await;
            }
        });
    }
}

#[async_trait]
impl ClientTransportProvider for MockTransportProvider {
    async fn connect<R: Handler>(
        &mut self,
        _connect_addr_string: String,
        handler: R,
        logger: Logger,
    ) -> Result<()> {
        let receiver = self.receiver.clone();
        async_std::task::spawn(async move {
            while let Ok(message) = receiver.recv().await {
                info!(
                    logger,
                    "MockTransportProvider received {:?}, forward to handler", message
                );
                handler.recv_non_ue_associated(message, &logger).await;
            }
        });
        Ok(())
    }
}
