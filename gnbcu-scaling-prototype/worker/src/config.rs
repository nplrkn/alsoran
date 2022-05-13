#[derive(Debug, Clone)]
pub struct Config {
    // The port to which the worker should bind its F1AP server.
    pub f1ap_bind_port: u16,

    // The port to which the worker should bind its callback server.
    pub callback_server_bind_port: u16,

    // Host and port of the URL that the worker should hand out for its callback service.
    // For example "example.com:1234".
    // When the worker is containerized, this is the external service name and port.
    // If none, then the host will be set to localhost and the port to the callback_server_bind_port.
    // See method callback_server_url().
    pub callback_server_url_host_port: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            f1ap_bind_port: 38472,            // TS38.472
            callback_server_bind_port: 23256, // arbitrary port used in testing
            callback_server_url_host_port: None,
        }
    }
}

impl Config {
    pub fn callback_server_base_path(&self) -> String {
        self.callback_server_url_host_port
            .clone()
            .unwrap_or_else(|| format!("http://localhost:{}", self.callback_server_bind_port))
    }
}
