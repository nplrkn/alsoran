use async_net::AsyncToSocketAddrs;
use async_std::prelude::Stream;
use async_std::task::{Context, Poll};
use std::io::Result;
use std::pin::Pin;

pub struct SctpListener {}
impl SctpListener {
    pub async fn bind<A: AsyncToSocketAddrs>(_addr: A) -> Result<SctpListener> {
        unimplemented!()
    }

    /// Returns an infinite stream of incoming connections.
    fn incoming(&self) -> Incoming<'_> {
        unimplemented!()
    }
}

/// A stream of incoming SCTP associations.
pub struct Incoming<'a> {
    _incoming: Pin<Box<dyn Stream<Item = Result<SctpAssociation>> + Send + Sync + 'a>>,
}

impl Stream for Incoming<'_> {
    type Item = Result<SctpAssociation>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        unimplemented!();
    }
}

// An SCTP assocation.
#[derive(Debug, Clone)]
pub struct SctpAssociation {}

impl SctpAssociation {
    // See https://docs.rs/async-net/1.6.1/async_net/struct.TcpStream.html
    pub async fn establish<A: AsyncToSocketAddrs>(_addr: A) -> Result<SctpAssociation> {
        unimplemented!();
    }

    pub async fn send(&self, _buf: &[u8], _stream_id: u32) -> Result<usize> {
        unimplemented!();
    }

    pub async fn recv(&self, _buf: &mut [u8]) -> Result<usize> {
        unimplemented!();
    }
}
