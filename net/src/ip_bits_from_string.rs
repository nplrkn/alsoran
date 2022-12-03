use async_net::{AddrParseError, IpAddr};
use bitvec::prelude::*;

pub fn ip_bits_from_string(addr: &String) -> Result<BitVec<u8, Msb0>, AddrParseError> {
    Ok(match addr.parse()? {
        IpAddr::V4(x) => BitVec::<_, Msb0>::from_slice(&x.octets()),
        IpAddr::V6(x) => BitVec::<_, Msb0>::from_slice(&x.octets()),
    })
}
