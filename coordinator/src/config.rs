//! config - the config of the GNB-CU Coordinator

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
    pub amf_address: String,

    // Worker refresh interval
    pub worker_refresh_interval_secs: u16,

    // Fast start - used to skip over initial learning time
    pub fast_start: bool,
}

impl Default for ConnectionControlConfig {
    fn default() -> Self {
        ConnectionControlConfig {
            amf_address: "127.0.0.1".to_string(),
            worker_refresh_interval_secs: 30,
            fast_start: false,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            bind_port: 43521, // TS38.472
            connection_control_config: ConnectionControlConfig::default(),
        }
    }
}
