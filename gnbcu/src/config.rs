//! config - the config of a GNB-CU

#[derive(Debug, Clone)]
pub struct Config {
    // The port to which the worker should bind its F1AP server.
    pub f1ap_bind_port: u16,

    // The port to which the worker should bind its E1AP server.
    pub e1ap_bind_port: u16,

    // The AMF "<address>:<port>"
    pub amf_address: String,

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
            amf_address: "127.0.0.1:38412".to_string(),
            initial_ue_ttl_secs: 5,
            ue_ttl_secs: 86_400, // a day
            name: Some("Alsoran".to_string()),
            plmn: vec![0x2, 0xf8, 0x39],
        }
    }
}
