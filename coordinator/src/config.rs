//! config - the config of the GNB-CU Coordinator

use connection_api::models::TransportAddress;

#[derive(Debug, Clone)]
pub struct Config {
    // The port on which to serve the coordination API.
    pub bind_port: u16,

    // AMF address
    pub connection_control_config: ConnectionControlConfig,
}

#[derive(Debug, Clone)]
pub struct ConnectionControlConfig {
    // AMF address
    pub amf_address: TransportAddress,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            bind_port: 43521, // TS38.472
            connection_control_config: ConnectionControlConfig {
                amf_address: TransportAddress::new("127.0.0.1".to_string(), 38412),
            },
        }
    }
}
