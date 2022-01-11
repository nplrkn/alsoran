#[derive(Debug, Clone)]
pub struct Config {
    // The port to which the worker should bind its callback server.
    pub callback_server_port: u16,

    // Host and port of the URL that the worker should hand out for its callback service.
    // When the worker is containerized, this is the external service name and port.
    // For example "example.com:1234".
    // If none, then the host will be set to localhost and the port to the callback_server_port.
    // See method callback_server_url().
    pub callback_server_host_port: Option<String>,
}

impl Config {
    pub fn callback_server_url(&self) -> String {
        self.callback_server_host_port
            .clone()
            .unwrap_or_else(|| format!("http://localhost:{}", self.callback_server_port))
    }
}
