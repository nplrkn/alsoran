use super::transport_provider::{ClientTransportProvider, Handler, Message, TransportProvider};
use anyhow::Result;
use async_channel::{Receiver, Sender};
use async_std::task::JoinHandle;
use async_trait::async_trait;
use slog::{trace, Logger};

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
        trace!(logger, "MockTransportProvider send message {:?}", message);
        self.sender.send(message).await.unwrap();
        Ok(())
    }
}

#[async_trait]
impl ClientTransportProvider for MockTransportProvider {
    async fn maintain_connection<R: Handler>(
        &self,
        _connect_addr_string: String,
        handler: R,
        logger: Logger,
    ) -> Result<JoinHandle<()>> {
        let receiver = self.receiver.clone();
        Ok(async_std::task::spawn(async move {
            while let Ok(message) = receiver.recv().await {
                trace!(
                    logger,
                    "MockTransportProvider received {:?}, forward to handler",
                    message
                );
                handler.recv_non_ue_associated(message, &logger).await;
            }
        }))
    }
}
