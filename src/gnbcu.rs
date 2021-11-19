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

    use super::*;

    impl<T: TransportProvider, F: TransportProvider> GNBCU<T, F> {
        pub async fn test_default(ngap_transport: T, f1_transport: F) -> GNBCU<T, F> {
            GNBCU::new(ngap_transport, f1_transport).await.unwrap()
        }
    }

    #[async_std::test]
    async fn initial_access_procedure() {
        // Creating a GNBCU will use a real SCTP transport.  However we can also create it with a mock tranport that
        // uses channels instead.
        println!("initial access procedure");

        let (mock_ngap_transport_provider, send_ngap, receive_ngap) =
            MockServerTransportProvider::new();
        let (mock_f1_transport_provider, send_f1, receive_f1) = MockServerTransportProvider::new();

        GNBCU::test_default(mock_ngap_transport_provider, mock_f1_transport_provider).await;
        let message: Vec<u8> = "hello world".into();
        println!("send in message");
        send_ngap.send(message).await.unwrap();
        let message = receive_f1.recv().await.unwrap();
        println!("Got F1 message {:?}, now send it back in", message);
        send_f1.send(message).await.unwrap();
        let message = receive_ngap.recv().await.unwrap();
        println!("Got NGAP message {:?}, done", message);
    }
}
