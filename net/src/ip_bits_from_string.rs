use anyhow::Result;
use async_net::IpAddr;
use bitvec::prelude::*;

pub fn ip_bits_from_string(addr: &str) -> Result<BitVec<u8, Msb0>> {
    Ok(match addr.parse()? {
        IpAddr::V4(x) => BitVec::<_, Msb0>::from_slice(&x.octets()),
        IpAddr::V6(x) => BitVec::<_, Msb0>::from_slice(&x.octets()),
    })
}
