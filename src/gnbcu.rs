use crate::ngap_receiver::NgapReceiver;
use crate::transport_provider::TransportProvider;
/// The gNB-CU.
use async_net::AsyncToSocketAddrs;

pub struct GNBCU<T> {
    ngap_receiver: NgapReceiver,
    ngap_transport_provider: T,
}

impl<T: TransportProvider> GNBCU<T> {
    pub async fn new<A: AsyncToSocketAddrs>(
        f1_listen_address: A,
        ngap_transport_provider: T,
    ) -> Result<GNBCU<T>, String> {
        let ngap_receiver = NgapReceiver {};
        Ok(GNBCU {
            ngap_receiver,
            ngap_transport_provider,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mock_transport_provider::MockServerTransportProvider;

    use super::*;
    use std::net::Ipv4Addr;

    // TODO move into F1 modules
    const F1AP_SCTP_DESTINATION_PORT: u16 = 38472;

    impl<T: TransportProvider> GNBCU<T> {
        pub async fn test_default(prov: T) -> GNBCU<T> {
            let f1_listen_address = (Ipv4Addr::UNSPECIFIED, F1AP_SCTP_DESTINATION_PORT);
            GNBCU::new(f1_listen_address, prov).await.unwrap()
        }
    }

    #[async_std::test]
    async fn initial_access_procedure() {
        // Creating a GNBCU will use a real SCTP transport.  However we can also create it with a mock tranport that
        // uses channels instead.
        let (mock_ngap_transport_provider, send_ngap, receive_ngap) =
            MockServerTransportProvider::<NgapReceiver>::new();
        //let mock_f1_transport_provider = MockServerTransportProvider::<F1>::new();

        let gnbcu = GNBCU::test_default(mock_ngap_transport_provider).await;
        let message: Vec<u8> = "hello world".into();
        send_ngap.send(Box::new(message));
    }
}
