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
            connection_style: ConnectionStyle::ConnectToAmf(ConnectionControlConfig::default()),
            initial_ue_ttl_secs: 5,
            ue_ttl_secs: 86_400, // a day
            name: Some("Alsoran".to_string()),
            plmn: vec![0x2, 0xf8, 0x39],
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConnectionStyle {
    ConnectToAmf(ConnectionControlConfig),
    ServeConnectionApi(ConnectionApiServerConfig),
}

#[derive(Debug, Clone)]
pub struct ConnectionApiServerConfig {
    // The port to bind the server to.
    pub bind_port: u16,

    // The base of the URL.  This could be formed from a service DNS name.
    pub base_path: String,
}
