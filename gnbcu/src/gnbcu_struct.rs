//! gnbcu_struct - the struct that implements the Gnbcu trait

use std::sync::Arc;

use super::config::ConnectionStyle;
use super::datastore::{UeState, UeStateStore};
use super::handlers::RrcHandler;
use super::rrc_transaction::{PendingRrcTransactions, RrcTransaction};
use super::Config;
use crate::handlers::connection_api::ConnectionApiHandler;
use crate::handlers::{E1apHandler, F1apHandler, NgapHandler};
use crate::{ConnectionApiServerConfig, Gnbcu};
use anyhow::Result;
use async_channel::Sender;
use async_std::sync::Mutex;
use async_trait::async_trait;
use coordination_api::models::{TransportAddress, WorkerInfo};
use coordination_api::{
    Api as CoordinationApi, Client as CoordinationApiClient, RefreshWorkerResponse,
};
use coordinator::Coordinator;
use f1ap::{DlRrcMessageTransfer, DlRrcMessageTransferProcedure, GnbCuUeF1apId, SrbId};
use net::{
    Indication, IndicationHandler, Procedure, RequestError, RequestProvider, SctpTransportProvider,
    ShutdownHandle, Stack,
};
use rrc::UlDcchMessage;
use slog::{debug, info, warn, Logger};
use stop_token::{StopSource, StopToken};
use swagger::{ApiError, AuthData, ContextBuilder, EmptyContext, Push, XSpanIdString};
use uuid::Uuid;

pub type ClientContext = swagger::make_context_ty!(
    ContextBuilder,
    EmptyContext,
    Option<AuthData>,
    XSpanIdString
);
#[derive(Clone)]
pub struct ConcreteGnbcu<A: CoordinationApi<ClientContext>, U: UeStateStore> {
    config: Config,
    ngap: Stack,
    f1ap: Stack,
    e1ap: Stack,
    ue_store: U,
    coordinator: A,
    logger: Logger,
    rrc_transactions: PendingRrcTransactions,
    shutdown_handles: Arc<Mutex<Vec<ShutdownHandle>>>,
}

// TS38.412, 7
// The Payload Protocol Identifier (ppid) assigned by IANA to be used by SCTP for the application layer protocol NGAP
// is 60, and 66 for DTLS over SCTP (IETF RFC 6083 [8]).
const NGAP_SCTP_PPID: u32 = 60;

// TS38.472, 7
// The Payload Protocol Identifier (ppid) assigned by IANA to be used by SCTP for the application layer protocol F1AP is 62,
// and 68 for DTLS over SCTP (IETF RFC 6083 [9]).
const F1AP_SCTP_PPID: u32 = 62;

// TS38.462
const E1AP_SCTP_PPID: u32 = 64;

pub fn spawn<U: UeStateStore>(
    config: Config,
    ue_store: U,
    logger: Logger,
) -> Result<ShutdownHandle> {
    info!(&logger, "Spawn GNBCU");
    let stop_source = StopSource::new();
    let stop_token = stop_source.token();

    let handle = match config.connection_style {
        // Run a combined GNBCU and Coordinator.
        ConnectionStyle::ConnectToAmf(ref connection_control_config) => {
            let (coordinator, receiver) = Coordinator::new(logger.clone());
            let gnbcu = ConcreteGnbcu::new(
                config.clone(),
                ue_store,
                logger.clone(),
                coordinator.clone(),
            );
            let handler = ConnectionApiHandler::new(gnbcu.clone(), logger);
            let coordinator_shutdown_handle = coordinator.start_with_local_api_provider(
                connection_control_config.clone(),
                receiver,
                handler,
            );
            async_std::task::spawn(async move {
                gnbcu
                    .serve(stop_token)
                    .await
                    .expect("Gnbcu startup failure");
                coordinator_shutdown_handle.graceful_shutdown().await;
            })
        }

        // Run a worker and serve the connection API so that it can be managed by the coordinator.
        ConnectionStyle::ServeConnectionApi(_) => {
            let coordinator = CoordinationApiClient::try_new_http("http://127.0.0.1:23156")?;
            let gnbcu = ConcreteGnbcu::new(config, ue_store, logger, coordinator);
            async_std::task::spawn(async move {
                gnbcu
                    .serve(stop_token)
                    .await
                    .expect("Gnbcu startup failure");
            })
        }
    };
    Ok(ShutdownHandle::new(handle, stop_source))
}

impl<A: Clone + Send + Sync + 'static + CoordinationApi<ClientContext>, U: UeStateStore>
    ConcreteGnbcu<A, U>
{
    fn new(config: Config, ue_store: U, logger: Logger, coordinator: A) -> ConcreteGnbcu<A, U> {
        ConcreteGnbcu {
            config,
            ngap: Stack::new(SctpTransportProvider::new()),
            f1ap: Stack::new(SctpTransportProvider::new()),
            e1ap: Stack::new(SctpTransportProvider::new()),
            ue_store,
            coordinator,
            logger,
            rrc_transactions: PendingRrcTransactions::new(),
            shutdown_handles: Arc::new(Mutex::new(Vec::new())),
        }
    }

    async fn serve(self, stop_token: StopToken) -> Result<()> {
        let f1ap_handle = self.serve_f1ap().await?;
        let e1ap_handle = self.serve_e1ap().await?;

        let connection_api_handle =
            if let ConnectionStyle::ServeConnectionApi(ConnectionApiServerConfig {
                bind_port,
                ..
            }) = self.config.connection_style
            {
                Some(self.serve_connection_api(bind_port).await?)
            } else {
                None
            };

        // Connect to the coordinator.  It will bring this worker into service by making calls to the
        // connection API.
        self.connect_to_coordinator().await;

        stop_token.await;

        if let Some(connection_api_handle) = connection_api_handle {
            connection_api_handle.graceful_shutdown().await;
        }
        f1ap_handle.graceful_shutdown().await;
        e1ap_handle.graceful_shutdown().await;
        Ok(())
    }

    async fn connect_to_coordinator(&self) {
        // // TODO here we just send a single refresh.  This will need to be updated to send one on a timer.
        // let stop_source = StopSource::new();
        // let stop_token = stop_source.token();
        // let (coordinator, control_task) = Coordinator::new(stop_token, self.logger.clone());

        let context: ClientContext = swagger::make_context!(
            ContextBuilder,
            EmptyContext,
            None as Option<AuthData>,
            XSpanIdString::default()
        );

        if let Err(e) = self.send_refresh_worker(&context).await {
            warn!(self.logger, "Failed initial refresh worker- {}", e);
        }
    }

    async fn send_refresh_worker(
        &self,
        context: &ClientContext,
    ) -> Result<RefreshWorkerResponse, ApiError> {
        let connected_amfs = self
            .ngap
            .remote_tnla_addresses()
            .await
            .iter()
            .map(|a| a.to_string())
            .collect();

        let connection_api_url = match &self.config.connection_style {
            ConnectionStyle::ConnectToAmf(_) => "".to_string(),
            ConnectionStyle::ServeConnectionApi(ConnectionApiServerConfig {
                base_path, ..
            }) => format!("{}/v1", base_path),
        };

        self.coordinator
            .refresh_worker(
                WorkerInfo {
                    worker_unique_id: Uuid::new_v4(),
                    connection_api_url,
                    f1_address: TransportAddress {
                        host: "127.0.0.1".to_string(),
                        port: self.config.f1ap_bind_port,
                    },
                    e1_address: TransportAddress {
                        host: "127.0.0.1".to_string(),
                        port: self.config.e1ap_bind_port,
                    },
                    connected_amfs,
                    connected_dus: vec![],
                    connected_ups: vec![],
                },
                &context,
            )
            .await
    }

    async fn serve_f1ap(&self) -> Result<ShutdownHandle> {
        let f1_listen_address = format!("0.0.0.0:{}", self.config.f1ap_bind_port).to_string();
        info!(
            &self.logger,
            "Listen for connection from DU on {}", f1_listen_address
        );
        let rrc_handler = RrcHandler::new(self.clone());
        self.f1ap
            .listen(
                f1_listen_address,
                F1AP_SCTP_PPID,
                F1apHandler::new_f1ap_application(self.clone(), rrc_handler),
                self.logger.clone(),
            )
            .await
    }
    async fn serve_e1ap(&self) -> Result<ShutdownHandle> {
        let e1_listen_address = format!("0.0.0.0:{}", self.config.e1ap_bind_port).to_string();
        info!(
            &self.logger,
            "Listen for connection from CU-UP on {}", e1_listen_address
        );
        self.e1ap
            .listen(
                e1_listen_address,
                E1AP_SCTP_PPID,
                E1apHandler::new_e1ap_application(self.clone()),
                self.logger.clone(),
            )
            .await
    }

    async fn serve_connection_api(&self, port: u16) -> Result<ShutdownHandle> {
        let addr = format!("127.0.0.1:{}", port).parse()?;
        crate::handlers::connection_api::serve(addr, self.clone(), self.logger.clone()).await
    }
}

#[async_trait]
impl<A: Clone + Send + Sync + 'static + CoordinationApi<ClientContext>, U: UeStateStore>
    UeStateStore for ConcreteGnbcu<A, U>
{
    async fn store(&self, k: u32, s: UeState, ttl_secs: usize) -> Result<()> {
        self.ue_store.store(k, s, ttl_secs).await
    }
    async fn retrieve(&self, k: &u32) -> Result<UeState> {
        self.ue_store.retrieve(k).await
    }
    async fn delete(&self, k: &u32) -> Result<()> {
        self.ue_store.delete(k).await
    }
}

#[async_trait]
impl<A: Clone + Send + Sync + 'static + CoordinationApi<ClientContext>, U: UeStateStore> Gnbcu
    for ConcreteGnbcu<A, U>
{
    fn config(&self) -> &Config {
        &self.config
    }
    async fn ngap_connect(&self, amf_address: &String) -> Result<()> {
        info!(&self.logger, "Maintain connection to AMF {}", amf_address);
        let shutdown_handle = self
            .ngap
            .connect(
                amf_address,
                NGAP_SCTP_PPID,
                NgapHandler::new_ngap_application(self.clone()),
                self.logger.clone(),
            )
            .await?;
        self.shutdown_handles.lock().await.push(shutdown_handle);
        Ok(())
    }

    async fn ngap_request<P: Procedure>(
        &self,
        r: P::Request,
        logger: &Logger,
    ) -> Result<P::Success, RequestError<P::Failure>> {
        <Stack as RequestProvider<P>>::request(&self.ngap, r, logger).await
    }
    async fn ngap_indication<P: Indication>(&self, r: P::Request, logger: &Logger) {
        <Stack as IndicationHandler<P>>::handle(&self.ngap, r, logger).await
    }

    async fn f1ap_request<P: Procedure>(
        &self,
        r: P::Request,
        logger: &Logger,
    ) -> Result<P::Success, RequestError<P::Failure>> {
        <Stack as RequestProvider<P>>::request(&self.f1ap, r, logger).await
    }
    async fn f1ap_indication<P: Indication>(&self, r: P::Request, logger: &Logger) {
        <Stack as IndicationHandler<P>>::handle(&self.f1ap, r, logger).await
    }

    async fn e1ap_request<P: Procedure>(
        &self,
        r: P::Request,
        logger: &Logger,
    ) -> Result<P::Success, RequestError<P::Failure>> {
        <Stack as RequestProvider<P>>::request(&self.e1ap, r, logger).await
    }
    async fn e1ap_indication<P: Indication>(&self, r: P::Request, logger: &Logger) {
        <Stack as IndicationHandler<P>>::handle(&self.e1ap, r, logger).await
    }

    /// Start a new RRC transaction.
    async fn new_rrc_transaction(&self, ue: &UeState) -> RrcTransaction {
        self.rrc_transactions.new_transaction(ue.key).await
    }

    /// Determine if this is a response to a local pending RRC transaction.
    async fn match_rrc_transaction(&self, ue_id: u32) -> Option<Sender<UlDcchMessage>> {
        // This is not a robust mechanism.  The calling task is only interested in the next matching
        // response to the RRC transactions it initiates, whereas we are giving it the next UlDcchMessage of any kind.
        // TODO
        self.rrc_transactions.match_transaction(ue_id).await
    }

    async fn send_rrc_to_ue(
        &self,
        ue: &UeState,
        srb_id: SrbId,
        rrc_container: f1ap::RrcContainer,
        logger: &Logger,
    ) {
        let dl_message = DlRrcMessageTransfer {
            gnb_cu_ue_f1ap_id: GnbCuUeF1apId(ue.key),
            gnb_du_ue_f1ap_id: ue.gnb_du_ue_f1ap_id.clone(),
            old_gnb_du_ue_f1ap_id: None,
            srb_id,
            execute_duplication: None,
            rrc_container,
            rat_frequency_priority_information: None,
            rrc_delivery_status_request: None,
            ue_context_not_retrievable: None,
            redirected_rrc_message: None,
            plmn_assistance_info_for_net_shar: None,
            new_gnb_cu_ue_f1ap_id: None,
            additional_rrm_priority_index: None,
        };

        debug!(&logger, "<< DlRrcMessageTransfer");
        DlRrcMessageTransferProcedure::call_provider(&self.f1ap, dl_message, logger).await
    }
}
