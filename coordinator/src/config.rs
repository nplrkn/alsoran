//! config - the config of the GNB-CU Coordinator

#[derive(Debug, Clone)]
pub struct Config {
    // The port on which to serve the coordination API.
    pub bind_port: u16,
}
