//! config - the config of a GNB-CU-UP

use std::net::{IpAddr, Ipv4Addr};

use asn1_per::nonempty;
use e1ap::{PlmnIdentity, SupportedPlmnsItem, SupportedPlmnsList};

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

impl Config {
    pub fn plmns(&self) -> SupportedPlmnsList {
        SupportedPlmnsList(nonempty![SupportedPlmnsItem {
            plmn_identity: PlmnIdentity([1, 2, 3]),
            slice_support_list: None,
            nr_cgi_support_list: None,
            qos_parameters_support_list: None,
            npn_support_info: None,
            extended_slice_support_list: None,
            extended_nr_cgi_support_list: None
        }])
    }
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
