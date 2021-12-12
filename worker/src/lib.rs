mod f1_handler;
use gnbcu::Gnbcu;
mod gnbcu;
#[cfg(test)]
mod mock_transport_provider;
mod ngap_handler;
mod sctp;
mod sctp_client_transport_provider;
use sctp_client_transport_provider::SctpClientTransportProvider;

mod transport_provider;
use slog::Logger;

// TS38.412, 7
// The Payload Protocol Identifier (ppid) assigned by IANA to be used by SCTP for the application layer protocol NGAP
// is 60, and 66 for DTLS over SCTP (IETF RFC 6083 [8]).
const NGAP_SCTP_PPID: u32 = 60;

// TS38.472, 7
// The Payload Protocol Identifier (ppid) assigned by IANA to be used by SCTP for the application layer protocol F1AP is 62,
// and 68 for DTLS over SCTP (IETF RFC 6083 [9]). The byte order of the ppid shall be big-endian.
const F1AP_NGAP_PPID: u32 = 62;

pub async fn run(logger: Logger) {
    let ngap_transport_provider = SctpClientTransportProvider::new(NGAP_SCTP_PPID);
    let f1_transport_provider = SctpClientTransportProvider::new(F1AP_NGAP_PPID);
    let _gnbcu = Gnbcu::new(ngap_transport_provider, f1_transport_provider, logger).await;
}
