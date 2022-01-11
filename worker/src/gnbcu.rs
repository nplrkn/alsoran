use crate::config::Config;
use crate::f1_handler::F1Handler;
use crate::ngap_handler::NgapHandler;
use crate::{ClientContext, F1ServerTransportProvider, NgapClientTransportProvider};
use anyhow::{anyhow, Result};
use async_std::task::JoinHandle;
use models::{RefreshWorkerReq, RefreshWorkerRsp, TransportAddress};
use node_control_api::client::callbacks::MakeService;
use node_control_api::{models, Api, RefreshWorkerResponse};
use slog::Logger;
use slog::{error, info, trace, warn};
use stop_token::{StopSource, StopToken};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::{AuthData, EmptyContext, Push, XSpanIdString};
use uuid::Uuid;

/// The gNB-CU.
#[derive(Debug, Clone)]
pub struct Gnbcu<T, F, C>
where
    T: NgapClientTransportProvider,
    F: F1ServerTransportProvider,
    C: Api<ClientContext> + Clone + Send + Sync + 'static,
{
    // TODO: why do these need to be pub?
    pub config: Config,
    pub ngap_transport_provider: T,
    pub f1_transport_provider: F,
    pub coordinator_client: C,
    pub logger: Logger,
}

impl<
        T: NgapClientTransportProvider,
        F: F1ServerTransportProvider,
        C: Api<ClientContext> + Send + Sync + Clone + 'static,
    > Gnbcu<T, F, C>
{
    pub fn new(
        config: Config,
        ngap_transport_provider: T,
        f1_transport_provider: F,
        coordinator_client: C,
        logger: &Logger,
    ) -> Gnbcu<T, F, C> {
        Gnbcu {
            config,
            ngap_transport_provider,
            f1_transport_provider,
            coordinator_client,
            logger: logger.clone(),
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

    fn start_callback_server(&self, stop_token: StopToken, logger: Logger) -> JoinHandle<()> {
        let addr = format!("0.0.0.0:{}", self.config.callback_server_bind_port)
            .parse()
            .expect("Failed to parse bind address"); // TODO
        let service = MakeService::new(self.clone());
        let service = MakeAllowAllAuthenticator::new(service, "cosmo");
        let service =
            node_control_api::server::context::MakeAddContext::<_, EmptyContext>::new(service);
        async_std::task::spawn(async move {
            let server = hyper::server::Server::bind(&addr)
                .serve(service)
                .with_graceful_shutdown(stop_token);
            if let Err(e) = server.await {
                error!(logger, "Server error: {}", e);
            } else {
                info!(logger, "Server graceful shutdown");
            }
        })
    }

    async fn serve(self, stop_token: StopToken) -> Result<()> {
        let logger = &self.logger;

        let context: ClientContext = swagger::make_context!(
            ContextBuilder,
            EmptyContext,
            None as Option<AuthData>,
            XSpanIdString::default()
        );

        trace!(logger, "Send initial refresh worker request");
        let response: RefreshWorkerResponse = self
            .coordinator_client
            .refresh_worker(
                RefreshWorkerReq {
                    callback_url: self.config.callback_server_url(),
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

        // Start node control callback server in a separate task.
        let callback_server_task = self.start_callback_server(stop_token.clone(), logger.clone());

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
        let address = format!("{}:{}", amf_address.host, amf_address.port.unwrap_or(38212));
        info!(logger, "Maintain connection to AMF {}", address);
        let connection_task = self
            .ngap_transport_provider
            .clone()
            .maintain_connection(address, ngap_handler, stop_token.clone(), logger.clone())
            .await?;

        let _f1_handler = F1Handler::new(self.clone());
        // gnbcu
        //     .f1_transport_provider
        //     .start_receiving(f1_handler, &logger.new(o!("component" => "F1")))
        //     .await;
        // info!(logger, "Started F1 handler");

        stop_token.await;

        // Wait for our tasks to terminate.
        connection_task.await;
        callback_server_task.await;

        info!(logger, "Stop");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::mock_coordinator::{MockCoordinator, NodeControlResponse};
    use also_net::MockTransportProvider;
    use anyhow::Result;
    use bitvec::vec::BitVec;
    use common::ngap::*;
    use models::RefreshWorkerRsp;
    use node_control_api::{models, RefreshWorkerResponse};
    use slog::{info, o};

    use super::*;

    #[async_std::test]
    async fn initial_access_procedure() -> Result<()> {
        let root_logger = common::logging::test_init();
        let logger = root_logger.new(o!("script" => "1"));

        // Create GNBCU with mock tranports + coordinator.
        let (mock_ngap_transport_provider, send_ngap, receive_ngap) =
            MockTransportProvider::<NgapPdu>::new();
        let (mock_f1_transport_provider, _send_f1, _receive_f1) =
            MockTransportProvider::<NgapPdu>::new();
        let (mock_coordinator, node_control_rsp, node_control_req) = MockCoordinator::new();

        let config = Config {
            callback_server_bind_port: 23256,
            callback_server_url_host_port: None,
        };

        let (stop_source, worker_task) = Gnbcu::new(
            config,
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

        // TODO - due to an apparent bug in the decoder, this has a spurious 00 on the end.
        //let precanned_ng_setup_response = hex::decode("20150031000004000100050100414d4600600008000002f839cafe0000564001ff005000100002f83900011008010203100811223300").unwrap();
        info!(logger, "Received NGAP NG Setup - send response");
        let ng_setup_response = NgapPdu::InitiatingMessage(InitiatingMessage {
            procedure_code: ProcedureCode(21),
            criticality: Criticality(Criticality::REJECT),
            value: InitiatingMessageValue::IdNgSetup(NgSetupRequest {
                protocol_i_es: NgSetupRequestProtocolIEs(vec![NgSetupRequestProtocolIEsItem {
                    id: ProtocolIeId(27),
                    criticality: Criticality(Criticality::REJECT),
                    value: NgSetupRequestProtocolIEsItemValue::IdGlobalRanNodeId(
                        GlobalRanNodeId::GlobalGnbId(GlobalGnbId {
                            plmn_identity: PlmnIdentity(vec![2, 3, 2, 1, 5, 6]),
                            gnb_id: GnbId::GnbId(BitString26(BitVec::from_element(0x10))),
                            ie_extensions: None,
                        }),
                    ),
                }]),
            }),
        });

        send_ngap.send(ng_setup_response).await.unwrap();

        info!(logger, "Start graceful shutdown of worker");
        drop(stop_source);
        worker_task.await;
        info!(logger, "Worker stopped");

        Ok(())
    }
}
