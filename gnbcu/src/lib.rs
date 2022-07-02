mod config;
mod handlers;
mod procedures;
mod rrc_transaction;
mod ue_context;
use anyhow::Result;
use async_channel::Sender;
use async_std::task::JoinHandle;
pub use config::Config;
use handlers::{f1ap, ngap, RrcHandler};
use net::{SctpTransportProvider, Stack};
use rrc::UlDcchMessage;
use rrc_transaction::{PendingRrcTransactions, RrcTransaction};
use slog::{info, trace, Logger};
use stop_token::{StopSource, StopToken};
use ue_context::UeContext;

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
    pub fn spawn(config: Config, logger: &Logger) -> Result<(StopSource, JoinHandle<()>)> {
        let gnbcu = Gnbcu {
            config,
            ngap: Stack::new(SctpTransportProvider::new(NGAP_SCTP_PPID)),
            f1ap: Stack::new(SctpTransportProvider::new(F1AP_SCTP_PPID)),
            logger: logger.clone(),
            rrc_transactions: PendingRrcTransactions::new(),
        };

        // TODO - replace with something like the model in net::TransportTasks.
        let stop_source = StopSource::new();
        let stop_token = stop_source.token();
        let task = async_std::task::spawn(async move {
            // Crash if this task exits.  (Otherwise the GNBCU process will be up but the only
            // thing running will be the initial thread waiting on signals.)
            gnbcu
                .serve(stop_token)
                .await
                .expect("Gnbcu startup failure");
        });
        Ok((stop_source, task))
    }

    async fn serve(self, stop_token: StopToken) -> Result<()> {
        let logger = &self.logger;
        let amf_address = "127.0.0.1:38412".to_string();
        info!(logger, "Maintain connection to AMF {}", amf_address);
        let ngap_transport = self
            .ngap
            .connect(amf_address, ngap::new(self.clone()), logger.clone())
            .await?;
        let f1_listen_address = format!("0.0.0.0:{}", self.config.f1ap_bind_port).to_string();
        info!(
            logger,
            "Listen for connection from DU on {}", f1_listen_address
        );
        let rrc_handler = RrcHandler::new(self.clone());
        let f1_transport = self
            .f1ap
            .listen(
                f1_listen_address,
                f1ap::new(self.clone(), rrc_handler),
                logger.clone(),
            )
            .await?;

        // Wait for our tasks to terminate.
        stop_token.await;
        ngap_transport.graceful_shutdown().await;
        f1_transport.graceful_shutdown().await;

        trace!(logger, "Server task finished");
        Ok(())
    }

    pub async fn connected_amf_change(&self, _logger: &Logger) {
        // TODO
    }

    // Start a new Rrc transaction.  This is not a robust long term mechnanism, since the
    // calling task is only interested in the next matching response to the Rrc transactions it initiates
    // whereas we are curently giving it the next UlDcchMessage of any kind.
    // TODO
    pub async fn new_rrc_transaction(&self, ue: &UeContext) -> RrcTransaction {
        self.rrc_transactions
            .new_transaction(ue.gnb_cu_ue_f1ap_id.0)
            .await
    }

    pub async fn match_rrc_transaction(&self, ue: &UeContext) -> Option<Sender<UlDcchMessage>> {
        self.rrc_transactions
            .match_transaction(ue.gnb_cu_ue_f1ap_id.0)
            .await
    }
}
