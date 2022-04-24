mod f1ap_handler;
mod ngap_handler;
mod node_control_callback_server;
use std::sync::Arc;

use crate::config::Config;
use crate::ClientContext;
use anyhow::{anyhow, Result};
use async_std::sync::Mutex;
use async_std::task::JoinHandle;
use models::{RefreshWorkerReq, RefreshWorkerRsp, TransportAddress};
use net::{SharedTransactions, TransactionReceiver, TransactionSender, TransportProvider};
use node_control_api::{models, Api, RefreshWorkerResponse};
use slog::Logger;
use slog::{info, trace, warn};
use stop_token::{StopSource, StopToken};
use swagger::{ApiError, AuthData, EmptyContext, Push, XSpanIdString};
use uuid::Uuid;

/// The gNB-CU.
#[derive(Clone)]
pub struct Gnbcu<N, F, C>
where
    N: TransportProvider,
    F: TransportProvider,
    C: Api<ClientContext> + Clone + Send + Sync + 'static,
{
    config: Config,
    worker_uuid: Uuid,
    ngap_transport_provider: TransactionSender<N>, // TODO rename to ngap?
    f1ap_transport_provider: TransactionSender<F>, // TODO rename to f1ap
    coordinator_client: C,
    ngap_transactions: SharedTransactions,
    f1ap_transactions: SharedTransactions,
    logger: Logger,
}

impl<
        N: TransportProvider,
        F: TransportProvider,
        C: Api<ClientContext> + Send + Sync + Clone + 'static,
    > Gnbcu<N, F, C>
{
    pub fn new(
        config: Config,
        ngap_transport_provider: N,
        f1ap_transport_provider: F,
        coordinator_client: C,
        logger: &Logger,
    ) -> Gnbcu<N, F, C> {
        let ngap_transactions = Arc::new(Mutex::new(Box::new(Vec::new())));
        let ngap_transport_provider =
            TransactionSender::new(ngap_transport_provider, ngap_transactions.clone());
        let f1ap_transactions = Arc::new(Mutex::new(Box::new(Vec::new())));
        let f1ap_transport_provider =
            TransactionSender::new(f1ap_transport_provider, f1ap_transactions.clone());
        Gnbcu {
            config,
            worker_uuid: Uuid::new_v4(),
            ngap_transport_provider,
            f1ap_transport_provider,
            coordinator_client,
            ngap_transactions,
            f1ap_transactions,
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

    async fn serve(self, stop_token: StopToken) -> Result<()> {
        let logger = &self.logger;
        trace!(logger, "Send initial refresh worker request");
        let response = self.send_refresh_worker().await?;

        // Start node control callback server in a separate task.
        let callback_server_task = self.start_callback_server(stop_token.clone())?;

        let ok_response = if let RefreshWorkerResponse::RefreshWorkerResponse(response) = response {
            trace!(logger, "Received refresh worker response");
            Ok(response)
        } else {
            warn!(logger, "Error response {:?}", response);
            Err(anyhow!("Coordinator failed request"))
        }?;

        let RefreshWorkerRsp { amf_addresses } = ok_response;
        let amf_address = &amf_addresses[0];

        let address = format!("{}:{}", amf_address.host, amf_address.port.unwrap_or(38212));
        info!(logger, "Maintain connection to AMF {}", address);
        let handler = TransactionReceiver::new(self.clone(), self.ngap_transactions.clone());
        let connection_task = self
            .ngap_transport_provider
            .transport_provider // TODO we don't want to reach inside like this
            .clone()
            .maintain_connection(address, handler, stop_token.clone(), logger.clone())
            .await?;

        let f1_server_task = self.start_f1ap_handler(stop_token.clone()).await?;

        stop_token.await;

        // Wait for our tasks to terminate.
        connection_task.await;
        callback_server_task.await;
        f1_server_task.await;

        info!(logger, "Stop");
        Ok(())
    }

    pub async fn connected_amf_change(&self, _logger: &Logger) {
        // TODO handle error
        let _response = self.send_refresh_worker().await;
    }

    async fn send_refresh_worker(&self) -> Result<RefreshWorkerResponse, ApiError> {
        let context: ClientContext = swagger::make_context!(
            ContextBuilder,
            EmptyContext,
            None as Option<AuthData>,
            XSpanIdString::default()
        );

        let connected_amfs = self
            .ngap_transport_provider
            .transport_provider // TODO we don't want to reach inside like this
            .remote_tnla_addresses()
            .await
            .iter()
            .map(|a| a.to_string())
            .collect();

        // Here, we work around a bug in the Rust OpenAPI generator which produces inconsistent code for the callback client and server.
        // The callback client needs to pass in a full URL, whereas the callback server matches the URL path after the "v1".
        // Here we build a URL to the coordinator that it can pass into its client, but that will also pass the regex running on
        // the (worker's) server.
        let callback_url = format!("{}/v1/trigger", self.config.callback_server_base_path());

        self.coordinator_client
            .refresh_worker(
                RefreshWorkerReq {
                    callback_url,
                    worker_unique_id: self.worker_uuid,
                    f1_address: TransportAddress {
                        host: "127.0.0.1".to_string(),
                        port: Some(345),
                    },
                    connected_amfs,
                    connected_dus: Vec::new(),
                },
                &context,
            )
            .await
    }
}

// Commented out this test.  It is not clear that the additional cost of maintaining the MockTransportProvider
// gives us any value compared to using a live SCTP transport provider.  Faster tests?
// #[cfg(test)]
// mod tests {
//     use crate::mock_coordinator::{MockCoordinator, NodeControlResponse};
//     use net::MockTransportProvider;
//     use anyhow::Result;
//     use bitvec::vec::BitVec;
//     use ngap::*;
//     use models::RefreshWorkerRsp;
//     use node_control_api::{models, RefreshWorkerResponse};
//     use slog::{info, o};

//     use super::*;

//     #[async_std::test]
//     async fn initial_access_procedure() -> Result<()> {
//         let root_logger = common::logging::test_init();
//         let logger = root_logger.new(o!("script" => "1"));

//         // Create GNBCU with mock tranports + coordinator.
//         let (mock_ngap_transport_provider, send_ngap, receive_ngap) =
//             MockTransportProvider::<NgapPdu>::new();
//         let (mock_f1ap_transport_provider, _send_f1, _receive_f1) =
//             MockTransportProvider::<NgapPdu>::new();
//         let (mock_coordinator, node_control_rsp, node_control_req) = MockCoordinator::new();

//         let config = Config {
//             callback_server_bind_port: 23256,
//             callback_server_url_host_port: None,
//         };

//         let (stop_source, worker_task) = Gnbcu::new(
//             config,
//             mock_ngap_transport_provider,
//             mock_f1ap_transport_provider,
//             mock_coordinator,
//             &root_logger,
//         )
//         .spawn();

//         info!(logger, "Wait for refresh worker request");
//         let _ignored = node_control_req.recv().await?;
//         info!(logger, "Received refresh worker request - send response");
//         node_control_rsp
//             .send(NodeControlResponse::RefreshWorkerResponse(
//                 RefreshWorkerResponse::RefreshWorkerResponse(RefreshWorkerRsp {
//                     amf_addresses: vec![TransportAddress {
//                         host: "6.7.8.9".to_string(),
//                         port: Some(10),
//                     }],
//                 }),
//             ))
//             .await?;

//         info!(logger, "Wait for NG Setup");
//         let _ignored_ngap = receive_ngap.recv().await.unwrap();

//         // TODO - due to an apparent bug in the decoder, this has a spurious 00 on the end.
//         //let precanned_ng_setup_response = hex::decode("20150031000004000100050100414d4600600008000002f839cafe0000564001ff005000100002f83900011008010203100811223300").unwrap();
//         info!(logger, "Received NGAP NG Setup - send response");
//         let ng_setup_response = NgapPdu::InitiatingMessage(InitiatingMessage {
//             procedure_code: ProcedureCode(21),
//             criticality: Criticality(Criticality::REJECT),
//             value: InitiatingMessageValue::IdNgSetup(NgSetupRequest {
//                 protocol_i_es: NgSetupRequestProtocolIEs(vec![NgSetupRequestProtocolIEsItem {
//                     id: ProtocolIeId(27),
//                     criticality: Criticality(Criticality::REJECT),
//                     value: NgSetupRequestProtocolIEsItemValue::IdGlobalRanNodeId(
//                         GlobalRanNodeId::GlobalGnbId(GlobalGnbId {
//                             plmn_identity: PlmnIdentity(vec![2, 3, 2, 1, 5, 6]),
//                             gnb_id: GnbId::GnbId(BitString26(BitVec::from_element(0x10))),
//                             ie_extensions: None,
//                         }),
//                     ),
//                 }]),
//             }),
//         });

//         send_ngap.send(ng_setup_response).await.unwrap();

//         info!(logger, "Start graceful shutdown of worker");
//         drop(stop_source);
//         worker_task.await;
//         info!(logger, "Worker stopped");

//         Ok(())
//     }
// }
