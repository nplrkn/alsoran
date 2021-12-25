use super::try_io::try_io;
use super::SctpAssociation;
use anyhow::{anyhow, Result};
use async_io::Async;
use async_std::task::{Context, Poll};
use futures_lite::future::FutureExt;
use libc::{accept, bind, listen, socket, AF_INET, IPPROTO_SCTP, SOCK_STREAM};
use os_socketaddr::OsSocketAddr;
use slog::Logger;
use std::{io::Error, pin::Pin};

pub struct SctpListener {
    fd: i32,
    ppid: u32,
    logger: Logger,
}
impl SctpListener {
    pub fn new_listen(
        addr: OsSocketAddr,
        ppid: u32,
        backlog: i32,
        logger: Logger,
    ) -> Result<SctpListener> {
        // Get a socket and immediately wrap it in a SctpListener to ensure it gets closed
        // properly in the drop function if something fails later in this function.
        let fd = try_io!(socket(AF_INET, SOCK_STREAM, IPPROTO_SCTP), "socket")?;
        let listener = SctpListener { fd, ppid, logger };
        try_io!(bind(fd, addr.as_ptr(), addr.len()), "bind")?;
        try_io!(listen(fd, backlog), "listen")?;
        Ok(listener)
    }

    async fn accept_next(&self) -> Result<SctpAssociation> {
        Async::new(self.fd)?.readable().await?;
        let mut addr = OsSocketAddr::new();
        let mut len = addr.len();
        let assoc_fd = try_io!(accept(self.fd, addr.as_mut_ptr(), &mut len), "accept")?;
        SctpAssociation::from_accepted(assoc_fd, self.ppid, &self.logger)
    }
}

impl async_std::stream::Stream for SctpListener {
    type Item = SctpAssociation;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let future_assoc = async { self.accept_next().await.ok() };
        futures::pin_mut!(future_assoc);
        future_assoc.poll(cx)
    }
}

impl Drop for SctpListener {
    fn drop(&mut self) {
        unsafe { libc::close(self.fd) };
    }
}
