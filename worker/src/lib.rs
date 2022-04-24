use gnbcu::Gnbcu;
pub mod config;
mod gnbcu;
#[cfg(test)]
mod mock_coordinator;
use anyhow::Result;
use async_std::task::JoinHandle;
pub use config::Config;
use net::SctpTransportProvider;
use node_control_api::Client;
use slog::{info, Logger};
use stop_token::StopSource;
use swagger::{AuthData, ContextBuilder, EmptyContext, XSpanIdString};

pub type ClientContext = swagger::make_context_ty!(
    ContextBuilder,
    EmptyContext,
    Option<AuthData>,
    XSpanIdString
);

// TS38.412, 7
// The Payload Protocol Identifier (ppid) assigned by IANA to be used by SCTP for the application layer protocol NGAP
// is 60, and 66 for DTLS over SCTP (IETF RFC 6083 [8]).
const NGAP_SCTP_PPID: u32 = 60;

// TS38.472, 7
// The Payload Protocol Identifier (ppid) assigned by IANA to be used by SCTP for the application layer protocol F1AP is 62,
// and 68 for DTLS over SCTP (IETF RFC 6083 [9]).
const F1AP_NGAP_PPID: u32 = 62;

pub fn spawn(config: Config, logger: Logger) -> Result<(StopSource, JoinHandle<()>)> {
    info!(logger, "Worker instance start");
    let ngap_transport_provider = SctpTransportProvider::new(NGAP_SCTP_PPID);
    let f1ap_transport_provider = SctpTransportProvider::new(F1AP_NGAP_PPID);

    let base_path = "http://127.0.0.1:23156";

    let coordinator_client = Client::try_new_http(base_path)?;

    Ok(Gnbcu::new(
        config,
        ngap_transport_provider,
        f1ap_transport_provider,
        coordinator_client,
        &logger,
    )
    .spawn())
}
