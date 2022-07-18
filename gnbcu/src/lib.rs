mod config;
mod datastore;
mod handlers;
mod procedures;
mod rrc_transaction;
use anyhow::Result;
use async_channel::Sender;
pub use config::Config;
use datastore::UeState;
use handlers::RrcHandler;
use net::{SctpTransportProvider, ShutdownHandle, Stack};
use rrc::UlDcchMessage;
use rrc_transaction::{PendingRrcTransactions, RrcTransaction};
use slog::{info, Logger};
use stop_token::{StopSource, StopToken};

use crate::handlers::{F1apHandler, NgapHandler};

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
    logger: Logger,
    rrc_transactions: PendingRrcTransactions,
}

impl Gnbcu {
    pub fn spawn(config: Config, logger: &Logger) -> Result<ShutdownHandle> {
        let gnbcu = Gnbcu {
            config,
            ngap: Stack::new(SctpTransportProvider::new(NGAP_SCTP_PPID)),
            f1ap: Stack::new(SctpTransportProvider::new(F1AP_SCTP_PPID)),
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

    /// Start a new RRC transaction.
    pub async fn new_rrc_transaction(&self, ue: &UeState) -> RrcTransaction {
        self.rrc_transactions
            .new_transaction(ue.gnb_cu_ue_f1ap_id.0)
            .await
    }

    /// Determine if this is a response to a local pending RRC transaction.
    pub async fn match_rrc_transaction(&self, ue: &UeState) -> Option<Sender<UlDcchMessage>> {
        // This is not a robust mechanism.  The calling task is only interested in the next matching
        // response to the RRC transactions it initiates, whereas we are giving it the next UlDcchMessage of any kind.
        // TODO
        self.rrc_transactions
            .match_transaction(ue.gnb_cu_ue_f1ap_id.0)
            .await
    }
}
