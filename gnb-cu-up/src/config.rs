//! config - the config of a GNB-CU

use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Clone)]
pub struct Config {
    // The IP address that the worker binds all of it listen ports to.  If there is only one worker
    // running on the system, this may be omitted.  To test multiple workers running on a
    // single system, each can be given a different 127.0.0.0/8 IP address.
    pub ip_address: Option<IpAddr>,

    // Set to the IP address of the GNB-CU-CP.
    pub cp_ip_address: IpAddr,

    // Human readable name of this GNB-CU-UP.
    pub name: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ip_address: None,
            cp_ip_address: Ipv4Addr::LOCALHOST.into(),
            name: Some("Alsoran UP".to_string()),
        }
    }
}
