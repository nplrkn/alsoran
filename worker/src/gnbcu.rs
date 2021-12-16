use crate::f1_handler::F1Handler;
use crate::ngap_handler::NgapHandler;
use crate::transport_provider::{ClientTransportProvider, TransportProvider};
use anyhow::{anyhow, Result};
use async_std::task::JoinHandle;
use slog::Logger;
use slog::{info, o, trace, warn};
use stop_token::{StopSource, StopToken};

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
    pub logger: Logger,
}

impl<
        T: ClientTransportProvider,
        F: TransportProvider,
        C: Api<ClientContext> + Send + Sync + Clone + 'static,
    > Gnbcu<T, F, C>
{
    pub fn new(
        ngap_transport_provider: T,
        f1_transport_provider: F,
        coordinator_client: C,
        logger: &Logger,
    ) -> Gnbcu<T, F, C> {
        let logger = logger.new(o!("gnbcu" => 1));
        Gnbcu {
            ngap_transport_provider,
            f1_transport_provider,
            coordinator_client,
            logger,
        }
    }

    pub fn spawn(self) -> (StopSource, JoinHandle<()>) {
        let stop_source = StopSource::new();
        let stop_token = stop_source.token();
        let task = async_std::task::spawn(async move {
            // Crash if this task exits, because there is no recovery loop.
            self.serve(stop_token).await.expect("Fatal error in worker");
        });
        (stop_source, task)
    }

    async fn serve(self, stop_token: StopToken) -> Result<()> {
        let logger = &self.logger;
        info!(logger, "Start");

        // Get the AMF address from the Coordinator.
        // Create client for talking to the coordinator.
        let context: ClientContext = swagger::make_context!(
            ContextBuilder,
            EmptyContext,
            None as Option<AuthData>,
            XSpanIdString::default()
        );

        trace!(logger, "Send refresh worker request");
        let response: RefreshWorkerResponse = self
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
            trace!(logger, "Received refresh worker response");
            Ok(response)
        } else {
            warn!(logger, "Error response {:?}", response);
            Err(anyhow!("Coordinator failed request"))
        }?;

        let RefreshWorkerRsp { amf_addresses } = ok_response;
        let amf_address = &amf_addresses[0];

        let ngap_handler = NgapHandler::new(self.clone());
        self.ngap_transport_provider
            .maintain_connection(
                format!("{}:{}", amf_address.host, amf_address.port.unwrap_or(38212)),
                ngap_handler,
                logger.new(o!("NGAP handler"=>1))
            )
            .await?;

        trace!(logger, "Send NG Setup");
        let precanned_ng_setup = hex::decode("00150035000004001b00080002f83910000102005240090300667265653567630066001000000000010002f839000010080102030015400140").unwrap();
        self.ngap_transport_provider
            .send_message(precanned_ng_setup, &logger)
            .await
            .unwrap();

        let _f1_handler = F1Handler::new(self.clone());
        // gnbcu
        //     .f1_transport_provider
        //     .start_receiving(f1_handler, &logger.new(o!("component" => "F1")))
        //     .await;
        // info!(logger, "Started F1 handler");

        stop_token.await;
        info!(logger, "Stop");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mock_coordinator::{MockCoordinator, NodeControlResponse},
        mock_transport_provider::MockTransportProvider,
    };
    use anyhow::Result;
    use models::RefreshWorkerRsp;
    use node_control_api::{models, RefreshWorkerResponse};
    use slog::info;

    use super::*;

    #[async_std::test]
    async fn initial_access_procedure() -> Result<()> {
        let root_logger = common::logging::test_init();
        let logger = root_logger.new(o!("script" => "1"));

        // Create GNBCU with mock tranports + coordinator.
        let (mock_ngap_transport_provider, send_ngap, receive_ngap) = MockTransportProvider::new();
        let (mock_f1_transport_provider, _send_f1, _receive_f1) = MockTransportProvider::new();
        let (mock_coordinator, node_control_rsp, node_control_req) = MockCoordinator::new();

        let (stop_source, worker_task) = Gnbcu::new(
            mock_ngap_transport_provider,
            mock_f1_transport_provider,
            mock_coordinator,
            &root_logger,
        )
        .spawn();

        info!(logger, "Wait for refresh worker request");
        let _ignored = node_control_req.recv().await?;
        info!(logger, "Received refresh worker request - send response");
        node_control_rsp
            .send(NodeControlResponse::RefreshWorkerResponse(
                RefreshWorkerResponse::RefreshWorkerResponse(RefreshWorkerRsp {
                    amf_addresses: vec![TransportAddress {
                        host: "6.7.8.9".to_string(),
                        port: Some(10),
                    }],
                }),
            ))
            .await?;

        info!(logger, "Wait for NG Setup");
        let _ignored_ngap = receive_ngap.recv().await.unwrap();
        info!(logger, "Received NGAP NG Setup - send response");
        send_ngap.send("incorrect NG setup response".into()).await.unwrap();

        info!(logger, "Start graceful shutdown of worker");
        drop(stop_source);
        worker_task.await;
        info!(logger, "Worker stopped");

        Ok(())
    }
}
