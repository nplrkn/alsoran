use async_net::addr::AsyncToSocketAddrs;
use std::io::Result;

struct SctpListener {}
impl SctpListener {
    pub async fn bind<A: AsyncToSocketAddrs>(addr: A) -> Result<SctpListener> {
        unimplemented!();
    }

    /// Returns an infinite stream of incoming connections.
    pub fn incoming(&self) -> Incoming<'_>;
}

/// A stream of incoming SCTP associations.
pub struct Incoming<'a> {
    incoming:
        Pin<Box<dyn Stream<Item = Result<SctpAssociation>> + Send + Sync + 'a>>,
}

impl Stream for Incoming<'_> {
    type Item = Result<SctpAssociation>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        unimplemented!();
    }
}

// An SCTP assocation.
struct SctpAssociation {}

impl SctpAssociation {
    // See https://docs.rs/async-net/1.6.1/async_net/struct.TcpStream.html
    pub async fn establish<A: AsyncToSocketAddrs>(addr: A) -> Result<SctpAssocation> {
        unimplemented!();
    }

    pub async fn send(&self, buf: &[u8], stream_id: u32) -> Result<usize> {
        unimplemented!();
    }

    pub async fn recv(&self, buf: &mut [u8]) -> Result<usize> {
        unimplemented!();
    }
}