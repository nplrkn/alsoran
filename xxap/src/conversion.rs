use crate::TransportLayerAddress;
use async_net::IpAddr;
use bitvec::prelude::*;

impl TryFrom<&str> for TransportLayerAddress {
    type Error = anyhow::Error;
    fn try_from(addr: &str) -> Result<Self, anyhow::Error> { 
        let bv = match addr.parse()? {
            IpAddr::V4(x) => BitVec::<_, Msb0>::from_slice(&x.octets()),
            IpAddr::V6(x) => BitVec::<_, Msb0>::from_slice(&x.octets()),
        };
        Ok(TransportLayerAddress(bv))
    }
}
