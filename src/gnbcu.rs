use crate::f1_handler::F1Handler;
use crate::ngap_handler::NgapHandler;
use crate::transport_provider::{ClientTransportProvider, TransportProvider};
use slog::Logger;
use slog::{info, o};

/// The gNB-CU.

#[derive(Debug, Clone)]
pub struct Gnbcu<T, F>
where
    T: ClientTransportProvider,
    F: TransportProvider,
{
    pub ngap_transport_provider: T,
    pub f1_transport_provider: F,
}

impl<T: ClientTransportProvider, F: TransportProvider> Gnbcu<T, F> {
    pub async fn new(
        ngap_transport_provider: T,
        f1_transport_provider: F,
        logger: Logger,
    ) -> Result<Gnbcu<T, F>, String> {
        let mut gnbcu = Gnbcu {
            ngap_transport_provider,
            f1_transport_provider,
        };

        let connect_addr_string = "127.0.0.1:38412".to_string();

        let ngap_handler = NgapHandler::new(gnbcu.clone());
        gnbcu
            .ngap_transport_provider
            .connect(
                connect_addr_string,
                ngap_handler,
                logger.new(o!("component" => "NGAP")),
            )
            .await?;
        info!(logger, "Started NGAP handler");

        let f1_handler = F1Handler::new(gnbcu.clone());
        gnbcu
            .f1_transport_provider
            .start_receiving(f1_handler, &logger.new(o!("component" => "F1")))
            .await;
        info!(logger, "Started F1 handler");

        Ok(gnbcu)
    }
}

#[cfg(test)]
mod tests {
    use crate::mock_transport_provider::MockTransportProvider;
    use slog::info;

    use super::*;

    #[async_std::test]
    async fn initial_access_procedure() {
        let root_logger = crate::logging::test_init();
        // Creating a Gnbcu will use a real SCTP transport.  However we can also create it with a mock tranport that
        // uses channels instead.

        let (mock_ngap_transport_provider, send_ngap, receive_ngap) = MockTransportProvider::new();
        let (mock_f1_transport_provider, send_f1, receive_f1) = MockTransportProvider::new();

        let _gnbcu = Gnbcu::new(
            mock_ngap_transport_provider,
            mock_f1_transport_provider,
            root_logger.clone(),
        )
        .await;
        let message_1: Vec<u8> = "hello world".into();
        let message_2: Vec<u8> = "goodbye cruel world".into();

        // ----------------- SUBTEST 1 --------------------------
        let logger = root_logger.new(o!("subtest" => 1));
        info!(logger, "Test script sends in message");

        // This sends the message into the channel.  It will be received in the
        // Gnbcu's NGAP handler.
        info!(logger, "Test script sends in NGAP message");
        send_ngap.send(message_1).await.unwrap();

        // The Gnbcu's NGAP handler then forwards the message out on the F1 provider.
        let message_1 = receive_f1.recv().await.unwrap();
        info!(logger, "Test script receives F1 message {:?}", message_1);

        // ----------------- SUBTEST 2 --------------------------
        let logger = root_logger.new(o!("subtest" => 2));

        // Send in a message on F1 and catch on the NGAP side.
        info!(logger, "Test script sends in F1 message");
        send_f1.send(message_2).await.unwrap();

        // THIS HANGS!!!!!!
        let message_2 = receive_ngap.recv().await.unwrap();
        info!(logger, "Test script receives NGAP message {:?}", message_2);

        // send_ngap.send(message_2).await.unwrap();
        // let message_2 = receive_f1.recv().await.unwrap();
        // send_f1.send(message_1).await.unwrap();
        // let message_1 = receive_ngap.recv().await.unwrap();
        info!(
            logger,
            "Test framework got messages {:?}, {:?}, done", message_1, message_2
        );
    }
}
