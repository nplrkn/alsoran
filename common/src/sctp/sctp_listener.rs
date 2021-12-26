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

struct FdGuard(i32);

pub fn new_listen(
    addr: OsSocketAddr,
    ppid: u32,
    backlog: i32,
    logger: Logger,
) -> impl Stream<Item = Result<SctpAssociation>> {
    try_stream! {
        let fd = FdGuard(try_io!(socket(AF_INET, SOCK_STREAM, IPPROTO_SCTP), "socket")?);
        try_io!(bind(fd.0, addr.as_ptr(), addr.len()), "bind")?;
        try_io!(listen(fd.0, backlog), "listen")?;
        loop {
            Async::new(fd.0)?.readable().await?;
            let mut addr = OsSocketAddr::new();
            let mut len = addr.len();
            let assoc_fd = try_io!(accept(fd.0, addr.as_mut_ptr(), &mut len), "accept")?;
            let assoc = SctpAssociation::from_accepted(assoc_fd, ppid, &logger)?;
            yield assoc;
        }
    }
}

impl Drop for FdGuard {
    fn drop(&mut self) {
        unsafe { libc::close(self.0) };
    }
}
