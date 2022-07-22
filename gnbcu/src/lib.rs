mod config;
mod datastore;
mod handlers;
mod procedures;
mod rrc_transaction;
use anyhow::Result;
use async_channel::Sender;
use async_trait::async_trait;
pub use config::Config;
use datastore::UeState;
use f1ap::{DlRrcMessageTransfer, DlRrcMessageTransferProcedure, SrbId};
use handlers::RrcHandler;
use net::{Indication, SctpTransportProvider, ShutdownHandle, Stack};
use rrc::UlDcchMessage;
use rrc_transaction::{PendingRrcTransactions, RrcTransaction};
use slog::{debug, info, Logger};
use stop_token::{StopSource, StopToken};

use crate::handlers::{F1apHandler, NgapHandler};

#[async_trait]
pub trait GnbcuOps: Send + Sync + Clone + 'static {
    fn ngap_stack(&self) -> &Stack;
    fn f1ap_stack(&self) -> &Stack;

    /// Start a new RRC transaction.
    async fn new_rrc_transaction(&self, ue: &UeState) -> RrcTransaction;

    /// Determine if this is a response to a local pending RRC transaction.
    async fn match_rrc_transaction(&self, ue: &UeState) -> Option<Sender<UlDcchMessage>>;

    async fn send_rrc_to_ue(&self, ue: UeState, rrc_container: f1ap::RrcContainer, logger: &Logger);
}

// TS38.412, 7
// The Payload Protocol Identifier (ppid) assigned by IANA to be used by SCTP for the application layer protocol NGAP
// is 60, and 66 for DTLS over SCTP (IETF RFC 6083 [8]).
const NGAP_SCTP_PPID: u32 = 60;

// TS38.472, 7
// The Payload Protocol Identifier (ppid) assigned by IANA to be used by SCTP for the application layer protocol F1AP is 62,
// and 68 for DTLS over SCTP (IETF RFC 6083 [9]).
const F1AP_SCTP_PPID: u32 = 62;

#[derive(Clone)]
pub struct Gnbcu {
    config: Config,
    ngap: Stack,
    f1ap: Stack,
    //ue_store: U,
    logger: Logger,
    rrc_transactions: PendingRrcTransactions,
}

impl Gnbcu {
    pub fn spawn_with_mock(config: Config, logger: &Logger) -> Result<ShutdownHandle> //, ue_store: U
    {
        let gnbcu = Gnbcu {
            config,
            ngap: Stack::new(SctpTransportProvider::new(NGAP_SCTP_PPID)),
            f1ap: Stack::new(SctpTransportProvider::new(F1AP_SCTP_PPID)),
            //ue_store,
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
    pub fn spawn(config: Config, logger: &Logger) -> Result<ShutdownHandle> {
        Self::spawn_with_mock(config, logger) //RedisUeStore::new()
    }

    async fn serve(self, stop_token: StopToken) -> Result<()> {
        let ngap_handle = self.connect_ngap().await?;
        let f1ap_handle = self.serve_f1ap().await?;
        stop_token.await;
        ngap_handle.graceful_shutdown().await;
        f1ap_handle.graceful_shutdown().await;
        Ok(())
    }

    async fn connect_ngap(&self) -> Result<ShutdownHandle> {
        let amf_address = "127.0.0.1:38412".to_string();
        info!(&self.logger, "Maintain connection to AMF {}", amf_address);
        self.ngap
            .connect(
                amf_address,
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
                F1apHandler::new_f1ap_application(self.clone(), rrc_handler),
                self.logger.clone(),
            )
            .await
    }
}

#[async_trait]
impl GnbcuOps for Gnbcu {
    fn ngap_stack(&self) -> &Stack {
        &self.ngap
    }
    fn f1ap_stack(&self) -> &Stack {
        &self.f1ap
    }

    /// Start a new RRC transaction.
    async fn new_rrc_transaction(&self, ue: &UeState) -> RrcTransaction {
        self.rrc_transactions
            .new_transaction(ue.gnb_cu_ue_f1ap_id.0)
            .await
    }

    /// Determine if this is a response to a local pending RRC transaction.
    async fn match_rrc_transaction(&self, ue: &UeState) -> Option<Sender<UlDcchMessage>> {
        // This is not a robust mechanism.  The calling task is only interested in the next matching
        // response to the RRC transactions it initiates, whereas we are giving it the next UlDcchMessage of any kind.
        // TODO
        self.rrc_transactions
            .match_transaction(ue.gnb_cu_ue_f1ap_id.0)
            .await
    }

    async fn send_rrc_to_ue(
        &self,
        ue: UeState,
        rrc_container: f1ap::RrcContainer,
        logger: &Logger,
    ) {
        let dl_message = DlRrcMessageTransfer {
            gnb_cu_ue_f1ap_id: ue.gnb_cu_ue_f1ap_id,
            gnb_du_ue_f1ap_id: ue.gnb_du_ue_f1ap_id,
            old_gnb_du_ue_f1ap_id: None,
            srb_id: SrbId(1),
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
