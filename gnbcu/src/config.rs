#[derive(Debug, Clone)]
pub struct Config {
    // The port to which the worker should bind its F1AP server.
    pub f1ap_bind_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            f1ap_bind_port: 38472, // TS38.472
        }
    }
}
