#[derive(Debug, Clone)]
pub struct Config {
    // The port to which the worker should bind its F1AP server.
    pub f1ap_bind_port: u16,

    // The port to which the worker should bind its E1AP server.
    pub e1ap_bind_port: u16,

    // TTL to set on the UE state during the initial access procedure
    pub initial_ue_ttl_secs: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            f1ap_bind_port: 38472, // TS38.472
            e1ap_bind_port: 38462, // TS38.462
            initial_ue_ttl_secs: 5,
        }
    }
}
