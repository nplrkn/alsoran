use crate::f1_handler::F1Handler;
use crate::ngap_handler::NgapHandler;
use crate::transport_provider::{ClientTransportProvider, TransportProvider};
use anyhow::{anyhow, Result};
use slog::Logger;
use slog::{info, o, warn};

use crate::ClientContext;
use models::{RefreshWorkerRsp, TransportAddress};
use node_control_api::{models, Api, RefreshWorkerResponse};
use uuid::Uuid;
// swagger::Has may be unused if there are no examples
use swagger::{AuthData, EmptyContext, Push, XSpanIdString};

/// The gNB-CU.

//trait CoordClient: Api<C: Send + Sync>;

#[derive(Debug, Clone)]
pub struct Gnbcu<T, F, C>
where
    T: ClientTransportProvider,
    F: TransportProvider,
    C: Api<ClientContext> + Clone + Send + Sync + 'static,
{
    pub ngap_transport_provider: T,
    pub f1_transport_provider: F,
    pub coordinator_client: C,
}

impl<
        T: ClientTransportProvider,
        F: TransportProvider,
        C: Api<ClientContext> + Send + Sync + Clone + 'static,
    > Gnbcu<T, F, C>
{
    pub async fn new(
        ngap_transport_provider: T,
        f1_transport_provider: F,
        coordinator_client: C,
        logger: Logger,
    ) -> Result<Gnbcu<T, F, C>> {
        let gnbcu = Gnbcu {
            ngap_transport_provider,
            f1_transport_provider,
            coordinator_client,
        };

        // Get the AMF address from the Coordinator.
        // Create client for talking to the coordinator.
        let context: ClientContext = swagger::make_context!(
            ContextBuilder,
            EmptyContext,
            None as Option<AuthData>,
            XSpanIdString::default()
        );

        let response: RefreshWorkerResponse = gnbcu
            .coordinator_client
            .refresh_worker(
                models::RefreshWorkerReq {
                    worker_unique_id: Uuid::new_v4(),
                    f1_address: TransportAddress {
                        host: "127.0.0.1".to_string(),
                        port: Some(345),
                    },
                    connected_amfs: Vec::new(),
                    connected_dus: Vec::new(),
                },
                &context,
            )
            .await?;

        let ok_response = if let RefreshWorkerResponse::RefreshWorkerResponse(response) = response {
            Ok(response)
        } else {
            warn!(logger, "Error response {:?}", response);
            Err(anyhow!("Coordinator failed request"))
        }?;

        let RefreshWorkerRsp { amf_addresses } = ok_response;
        let amf_address = &amf_addresses[0];

        let ngap_handler = NgapHandler::new(gnbcu.clone());
        gnbcu
            .ngap_transport_provider
            .maintain_connection(
                format!("{}:{}", amf_address.host, amf_address.port.unwrap_or(38212)),
                ngap_handler,
                logger.new(o!("component" => "NGAP")),
            )
            .await?;
        info!(logger, "Started NGAP handler");

        // let precanned_ng_setup = hex::decode("00150035000004001b00080002f83910000102005240090300667265653567630066001000000000010002f839000010080102030015400140").unwrap();
        // gnbcu
        //     .ngap_transport_provider
        //     .send_message(precanned_ng_setup, &logger)
        //     .await
        //     .unwrap();

        let _f1_handler = F1Handler::new(gnbcu.clone());
        // gnbcu
        //     .f1_transport_provider
        //     .start_receiving(f1_handler, &logger.new(o!("component" => "F1")))
        //     .await;
        // info!(logger, "Started F1 handler");

        Ok(gnbcu)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mock_coordinator::MockCoordinator, mock_transport_provider::MockTransportProvider,
    };
    use slog::info;

    use super::*;

    #[async_std::test]
    async fn initial_access_procedure() {
        let root_logger = common::logging::test_init();
        // Creating a Gnbcu will use a real SCTP transport.  However we can also create it with a mock tranport that
        // uses channels instead.

        let (mock_ngap_transport_provider, send_ngap, receive_ngap) = MockTransportProvider::new();
        let (mock_f1_transport_provider, send_f1, receive_f1) = MockTransportProvider::new();
        let mock_coordinator = MockCoordinator {};

        let _gnbcu = Gnbcu::new(
            mock_ngap_transport_provider,
            mock_f1_transport_provider,
            mock_coordinator,
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
