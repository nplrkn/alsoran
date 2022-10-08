//! gnbcu_struct - the struct that implements the Gnbcu trait

use super::datastore::{UeState, UeStateStore};
use super::handlers::RrcHandler;
use super::rrc_transaction::{PendingRrcTransactions, RrcTransaction};
use super::Config;
use anyhow::Result;
use async_channel::Sender;
use async_trait::async_trait;
use f1ap::{DlRrcMessageTransfer, DlRrcMessageTransferProcedure, GnbCuUeF1apId, SrbId};
use net::{
    Indication, IndicationHandler, Procedure, RequestError, RequestProvider, SctpTransportProvider,
    ShutdownHandle, Stack,
};
use rrc::UlDcchMessage;
use slog::{debug, info, Logger};
use stop_token::{StopSource, StopToken};

use crate::handlers::{E1apHandler, F1apHandler, NgapHandler};
use crate::Gnbcu;

#[derive(Clone)]
pub struct ConcreteGnbcu<U: UeStateStore> {
    config: Config,
    ngap: Stack,
    f1ap: Stack,
    e1ap: Stack,
    ue_store: U,
    logger: Logger,
    rrc_transactions: PendingRrcTransactions,
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

impl<U: UeStateStore> ConcreteGnbcu<U> {
    pub fn spawn(config: Config, ue_store: U, logger: &Logger) -> Result<ShutdownHandle> {
        let gnbcu = ConcreteGnbcu {
            config,
            ngap: Stack::new(SctpTransportProvider::new()),
            f1ap: Stack::new(SctpTransportProvider::new()),
            e1ap: Stack::new(SctpTransportProvider::new()),
            ue_store,
            logger: logger.clone(),
            rrc_transactions: PendingRrcTransactions::new(),
        };

        let stop_source = StopSource::new();
        let stop_token = stop_source.token();
        let handle = async_std::task::spawn(async move {
            // Crash if this task exits with an error.  (Otherwise the GNBCU process will be up but the only
            // thing running will be the initial thread waiting on signals.)
            gnbcu
                .serve(stop_token)
                .await
                .expect("Gnbcu startup failure");
        });
        Ok(ShutdownHandle::new(handle, stop_source))
    }

    async fn serve(self, stop_token: StopToken) -> Result<()> {
        let ngap_handle = self.connect_ngap().await?;
        let f1ap_handle = self.serve_f1ap().await?;
        let e1ap_handle = self.serve_e1ap().await?;
        stop_token.await;
        ngap_handle.graceful_shutdown().await;
        f1ap_handle.graceful_shutdown().await;
        e1ap_handle.graceful_shutdown().await;
        Ok(())
    }

    async fn connect_ngap(&self) -> Result<ShutdownHandle> {
        let amf_address = "127.0.0.1:38412".to_string();
        info!(&self.logger, "Maintain connection to AMF {}", amf_address);
        self.ngap
            .connect(
                amf_address,
                NGAP_SCTP_PPID,
                NgapHandler::new_ngap_application(self.clone()),
                self.logger.clone(),
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
}

#[async_trait]
impl<U: UeStateStore> UeStateStore for ConcreteGnbcu<U> {
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
impl<U: UeStateStore> Gnbcu for ConcreteGnbcu<U> {
    fn config(&self) -> &Config {
        &self.config
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
