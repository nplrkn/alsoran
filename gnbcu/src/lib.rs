mod config;
mod f1ap_handler;
mod ngap_handler;
mod rrc_handler;
mod rrc_response_binding;
mod ue_context;
use anyhow::Result;
use async_std::task::JoinHandle;
pub use config::Config;
use net::{SctpTransportProvider, Stack};
use rrc_handler::RrcHandler;
use rrc_response_binding::RrcResponseBinding;
use slog::{info, Logger};
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
}

impl Gnbcu {
    pub fn spawn(config: Config, logger: &Logger) -> Result<(StopSource, JoinHandle<()>)> {
        let gnbcu = Gnbcu {
            config,
            ngap: Stack::new(SctpTransportProvider::new(NGAP_SCTP_PPID)),
            f1ap: Stack::new(SctpTransportProvider::new(F1AP_SCTP_PPID)),
            logger: logger.clone(),
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
            .connect(amf_address, ngap_handler::new(self.clone()), logger.clone())
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
                f1ap_handler::new(self.clone(), rrc_handler),
                logger.clone(),
            )
            .await?;

        // Wait for our tasks to terminate.
        stop_token.await;
        ngap_transport.graceful_shutdown().await;
        f1_transport.graceful_shutdown().await;

        info!(logger, "Stop");
        Ok(())
    }

    pub async fn connected_amf_change(&self, _logger: &Logger) {
        // TODO
    }

    pub fn bind_rrc_ul_dcch(&self, _ue: &UeContext) -> RrcResponseBinding {
        todo!()
    }
}
