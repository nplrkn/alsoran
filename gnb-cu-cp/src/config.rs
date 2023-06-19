//! config - the config of a GNB-CU

pub use coordinator::ConnectionControlConfig;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Clone)]
pub struct Config {
    // The IP address that the worker binds all of it listen ports to. To test multiple workers running on a
    // single system, each can be given a different 127.0.0.0/8 IP address.
    pub ip_addr: IpAddr,

    // Set this to Autonomous to have a single worker that connects immediately to AMF on the given
    // IP address.
    //
    // Set this to Coordinated to enable external control of connection management.
    pub connection_style: ConnectionStyle,

    // TTL to set on the UE state during the initial access procedure
    pub initial_ue_ttl_secs: usize,

    // TTL to set on the UE state once UE is configured
    pub ue_ttl_secs: usize,

    // Human readable name signaled in NG Setup Request, E1 GnbCuUpE1SetupResponse and F1SetupResponse
    pub name: Option<String>,

    // PLMN
    pub plmn: [u8; 3],
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ip_addr: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            connection_style: ConnectionStyle::Autonomous(ConnectionControlConfig {
                fast_start: true,
                ..ConnectionControlConfig::default()
            }),
            initial_ue_ttl_secs: 5,
            ue_ttl_secs: 86_400, // a day
            name: Some("Alsoran".to_string()),
            plmn: [0x2, 0xf8, 0x39],
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
