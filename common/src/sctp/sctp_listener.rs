use super::try_io::try_io;
use super::SctpAssociation;
use anyhow::{anyhow, Result};
use async_io::Async;
use async_stream::try_stream;
use futures_core::stream::Stream;
use libc::{accept, bind, listen, socket, AF_INET, IPPROTO_SCTP, SOCK_STREAM};
use os_socketaddr::OsSocketAddr;
use slog::Logger;
use std::io::Error;

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
    ) -> impl Stream<Item = Result<SctpAssociation>> {
        try_stream! {
            // Get a socket and immediately wrap it in a SctpListener to ensure it gets closed
            // properly in the drop function if something fails later in this function.
            let fd = try_io!(socket(AF_INET, SOCK_STREAM, IPPROTO_SCTP), "socket")?;
            //let listener = SctpListener { fd, ppid, logger };
            try_io!(bind(fd, addr.as_ptr(), addr.len()), "bind")?;
            try_io!(listen(fd, backlog), "listen")?;
            Async::new(fd)?.readable().await?;
            let mut addr = OsSocketAddr::new();
            let mut len = addr.len();
            let assoc_fd = try_io!(accept(fd, addr.as_mut_ptr(), &mut len), "accept")?;
            let assoc = SctpAssociation::from_accepted(assoc_fd, ppid, &logger)?;
            yield assoc;
        }
    }
}

impl Drop for SctpListener {
    fn drop(&mut self) {
        unsafe { libc::close(self.fd) };
    }
}
