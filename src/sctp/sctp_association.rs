use super::Message;
use crate::sctp::sctp_c_bindings::socket;
use async_net::AsyncToSocketAddrs;
use std::io::Result;

// An SCTP assocation.
#[derive(Debug, Clone)]
pub struct SctpAssociation {}

impl SctpAssociation {
    // See https://docs.rs/async-net/1.6.1/async_net/struct.TcpStream.html
    pub async fn establish<A: AsyncToSocketAddrs>(_addr: A) -> Result<SctpAssociation> {
        unsafe {
            let s = socket(4, 5, 6);
        }
        unimplemented!();
    }

    // pub async fn send(&self, _buf: &[u8], _stream_id: u32) -> Result<usize> {
    //     unimplemented!();
    // }

    pub async fn recv_msg(&self) -> Result<Message> {
        unimplemented!();
    }
}
