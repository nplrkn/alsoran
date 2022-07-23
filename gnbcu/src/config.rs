use crate::handlers::RrcHandlerConfig;

#[derive(Debug, Clone)]
pub struct Config {
    // The port to which the worker should bind its F1AP server.
    pub f1ap_bind_port: u16,

    pub rrc_handler_config: RrcHandlerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            f1ap_bind_port: 38472, // TS38.472
            rrc_handler_config: RrcHandlerConfig::default(),
        }
    }
}
