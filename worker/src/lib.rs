mod f1_handler;
mod node_control_callback_server;
use gnbcu::Gnbcu;
mod gnbcu;
#[cfg(test)]
mod mock_coordinator;
mod ngap_handler;
use also_net::{
    ClientTransportProvider, Codec, MockTransportProvider, SctpTransportProvider,
    ServerTransportProvider, TransportProvider,
};
use async_std::task::JoinHandle;
use common::ngap::NgapPdu;
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

pub trait NgapClientTransportProvider:
    ClientTransportProvider<Pdu = NgapPdu> + TransportProvider<Pdu = NgapPdu>
{
}
impl<C> NgapClientTransportProvider for SctpTransportProvider<C, NgapPdu> where
    C: Codec<Pdu = NgapPdu>
{
}
impl NgapClientTransportProvider for MockTransportProvider<NgapPdu> {}

pub trait F1ServerTransportProvider:
    ServerTransportProvider<Pdu = NgapPdu> + TransportProvider<Pdu = NgapPdu>
{
}
impl<C> F1ServerTransportProvider for SctpTransportProvider<C, NgapPdu> where C: Codec<Pdu = NgapPdu>
{}
impl F1ServerTransportProvider for MockTransportProvider<NgapPdu> {}

pub fn spawn<N: Codec<Pdu = NgapPdu> + 'static, F: Codec<Pdu = NgapPdu> + 'static>(
    logger: Logger,
    ngap_codec: N,
    f1_codec: F,
) -> (StopSource, JoinHandle<()>) {
    info!(logger, "Start");
    let ngap_transport_provider = SctpTransportProvider::new(NGAP_SCTP_PPID, ngap_codec);
    let f1_transport_provider = SctpTransportProvider::new(F1AP_NGAP_PPID, f1_codec);

    let base_path = "http://127.0.0.1:23156";

    let coordinator_client = Client::try_new_http(base_path).expect("Failed to create HTTP client");

    Gnbcu::new(
        ngap_transport_provider,
        f1_transport_provider,
        coordinator_client,
        &logger,
    )
    .spawn()
}
