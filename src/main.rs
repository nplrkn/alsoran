mod f1_handler;
mod gnbcu;
mod logging;
#[cfg(test)]
mod mock_transport_provider;
mod ngap_handler;
mod sctp_client_transport_provider;
mod transport_provider;

use gnbcu::GNBCU;
use sctp_client_transport_provider::SctpClientTransportProvider;

#[async_std::main]
async fn main() {
    let root_logger = logging::init();

    let ngap_transport_provider = SctpClientTransportProvider::new();
    let f1_transport_provider = SctpClientTransportProvider::new();

    let _gnbcu = GNBCU::new(
        ngap_transport_provider,
        f1_transport_provider,
        root_logger.clone(),
    )
    .await;

    // Connect to AMF

    // Send NG Setup

    // Get reply

    // or

    // Maintain connection to AMF
}
