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

