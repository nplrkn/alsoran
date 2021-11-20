use crate::transport_provider::TransportProvider;

/// The gNB-CU.

#[derive(Debug, Clone)]
pub struct GNBCU<T, F> {
    ngap_transport_provider: T,
    f1_transport_provider: F,
}

impl<T: TransportProvider, F: TransportProvider> GNBCU<T, F> {
    pub async fn new(
        ngap_transport_provider: T,
        f1_transport_provider: F,
    ) -> Result<GNBCU<T, F>, String> {
        let gnbcu = GNBCU {
            ngap_transport_provider,
            f1_transport_provider,
        };

        // When we receive a message, call the callback
        let ngap_transport_provider = gnbcu.ngap_transport_provider.clone();
        let f1_transport_provider = gnbcu.f1_transport_provider.clone();
        async_std::task::spawn(async move {
            while let Some(message) = f1_transport_provider.recv_message().await {
                ngap_transport_provider.send_message(message).await.unwrap();
            }
        });

        let ngap_transport_provider = gnbcu.ngap_transport_provider.clone();
        let f1_transport_provider = gnbcu.f1_transport_provider.clone();
        async_std::task::spawn(async move {
            while let Some(message) = ngap_transport_provider.recv_message().await {
                f1_transport_provider.send_message(message).await.unwrap();
            }
        });

        Ok(gnbcu)
    }
}

#[cfg(test)]
mod tests {
    use crate::mock_transport_provider::MockServerTransportProvider;
    use slog::info;

    use super::*;

    impl<T: TransportProvider, F: TransportProvider> GNBCU<T, F> {
        pub async fn test_default(ngap_transport: T, f1_transport: F) -> GNBCU<T, F> {
            GNBCU::new(ngap_transport, f1_transport).await.unwrap()
        }
    }

    #[async_std::test]
    async fn initial_access_procedure() {
        let logger = crate::logging::init();
        // Creating a GNBCU will use a real SCTP transport.  However we can also create it with a mock tranport that
        // uses channels instead.

        let (mock_ngap_transport_provider, send_ngap, receive_ngap) =
            MockServerTransportProvider::new();
        let (mock_f1_transport_provider, send_f1, receive_f1) = MockServerTransportProvider::new();

        GNBCU::test_default(mock_ngap_transport_provider, mock_f1_transport_provider).await;
        let message_1: Vec<u8> = "hello world".into();
        let message_2: Vec<u8> = "goodbye cruel world".into();
        info!(logger, "send in message");
        send_ngap.send(message_1).await.unwrap();
        send_f1.send(message_2).await.unwrap();
        let message_2 = receive_ngap.recv().await.unwrap();
        send_ngap.send(message_2).await.unwrap();
        let message_1 = receive_f1.recv().await.unwrap();
        let message_2 = receive_f1.recv().await.unwrap();
        send_f1.send(message_1).await.unwrap();
        let message_1 = receive_ngap.recv().await.unwrap();
        info!(
            logger,
            "Got messages {:?}, {:?}, done", message_1, message_2
        );
    }
}
