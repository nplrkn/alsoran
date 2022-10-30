//! config - the config of the GNB-CU Coordinator

use connection_api::models::TransportAddress;

#[derive(Debug, Clone)]
pub struct Config {
    // The port on which to serve the coordination API.
    pub bind_port: u16,

    // AMF address
    pub amf_address: TransportAddress,
}
