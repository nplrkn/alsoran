use gnbcu::Gnbcu;
pub mod config;
mod gnbcu;
use anyhow::Result;
use async_std::task::JoinHandle;
pub use config::Config;
use net::SctpTransportProvider;
use slog::{info, Logger};
use stop_token::StopSource;
// use swagger::{AuthData, ContextBuilder, EmptyContext, XSpanIdString};

// pub type ClientContext = swagger::make_context_ty!(
//     ContextBuilder,
//     EmptyContext,
//     Option<AuthData>,
//     XSpanIdString
// );

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
    Gnbcu::spawn(
        config,
        SctpTransportProvider::new(NGAP_SCTP_PPID),
        SctpTransportProvider::new(F1AP_NGAP_PPID),
        &logger,
    )
}
