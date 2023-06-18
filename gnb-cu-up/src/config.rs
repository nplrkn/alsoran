//! config - the config of a GNB-CU-UP

use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Clone)]
pub struct Config {
    // The IP address that the CU-UP instance binds its SCTP E1 port to,
    // and uses for nanomsg pub-sub.
    pub local_ip_address: IpAddr,

    // The local userplane address used to terminate GTP-U.  For an HA cluster, this is
    // set to be the same on all instances, meaning they can interchangeably forward packets.
    pub userplane_ip_address: IpAddr,

    // IP address of the GNB-CU-CP.
    pub cp_ip_address: IpAddr,

    // Human readable name of this GNB-CU-UP.
    pub name: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            local_ip_address: Ipv4Addr::LOCALHOST.into(),
            userplane_ip_address: Ipv4Addr::LOCALHOST.into(),
            cp_ip_address: Ipv4Addr::LOCALHOST.into(),
            name: Some("Alsoran UP".to_string()),
        }
    }
}
