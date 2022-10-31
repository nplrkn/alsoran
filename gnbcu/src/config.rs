//! config - the config of a GNB-CU

pub use connection_api::models::TransportAddress;
pub use coordinator::ConnectionControlConfig;

#[derive(Debug, Clone)]
pub struct Config {
    // The port to which the worker should bind its F1AP server.
    pub f1ap_bind_port: u16,

    // The port to which the worker should bind its E1AP server.
    pub e1ap_bind_port: u16,

    // Set this to ConnectToAmf("<address>:<port>"") to have a single worker that
    // connects immediately to AMF on the given IP address and port.
    //
    // Set this to ServeConnectionApi(<bind_port>) to enable external control of
    // connection management.
    pub connection_style: ConnectionStyle,

    // TTL to set on the UE state during the initial access procedure
    pub initial_ue_ttl_secs: usize,

    // TTL to set on the UE state once UE is configured
    pub ue_ttl_secs: usize,

    // Human readable name signaled in NG Setup Request, E1 GnbCuUpE1SetupResponse and F1SetupResponse
    pub name: Option<String>,

    // PLMN
    pub plmn: Vec<u8>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            f1ap_bind_port: 38472, // TS38.472
            e1ap_bind_port: 38462, // TS38.462
            connection_style: ConnectionStyle::Autonomous(ConnectionControlConfig::default()),
            initial_ue_ttl_secs: 5,
            ue_ttl_secs: 86_400, // a day
            name: Some("Alsoran".to_string()),
            plmn: vec![0x2, 0xf8, 0x39],
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConnectionStyle {
    // Singleton worker that connects directly to the AMF.
    Autonomous(ConnectionControlConfig),

    // The worker can run as part of a cluster, serves the Connection API and relies on a
    // separate Coordinator to control its connections.
    Coordinated(WorkerConnectionManagementConfig),
}

#[derive(Debug, Clone)]
pub struct WorkerConnectionManagementConfig {
    // The port the worker will bind the Connection API to.
    pub connection_api_bind_port: u16,

    // The base of the URL of the Connection API served by a worker instance.  This will be
    // communicated to, and must be resolvable / reachable by the coordinator.
    // Example: "http://example.com:6007"
    pub connection_api_base_path: String,

    // The base path of URL that the worker will use to contact the coordinator.
    // Example: "http://example.com:12345"
    pub coordinator_base_path: String,
}
