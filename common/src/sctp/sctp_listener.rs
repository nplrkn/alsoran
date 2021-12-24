use super::try_io::try_io;
use super::SctpAssociation;
use anyhow::{anyhow, Result};
use async_io::Async;
use libc::{accept, bind, listen, socket, socklen_t, AF_INET, IPPROTO_SCTP, SOCK_STREAM};
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
    ) -> Result<SctpListener> {
        // Get a socket and immediately wrap it in a SctpListener to ensure it gets closed
        // properly in the drop function if something fails later in this function.
        let fd = try_io!(socket(AF_INET, SOCK_STREAM, IPPROTO_SCTP), "socket")?;
        let listener = SctpListener { fd, ppid, logger };
        try_io!(bind(fd, addr.as_ptr(), addr.len()), "bind")?;
        try_io!(listen(fd, backlog), "listen")?;
        Ok(listener)
    }

    pub async fn accept_next(&self) -> Result<SctpAssociation> {
        let async_fd = Async::new(self.fd).unwrap();
        async_fd.readable().await?;
        let mut sockaddr = unsafe { std::mem::zeroed::<libc::sockaddr>() };
        let mut restrict = std::mem::size_of::<libc::sockaddr>() as socklen_t;
        let assoc_fd = try_io!(
            accept(
                self.fd,
                &mut sockaddr as *mut _ as _,
                &mut restrict as *mut _ as _
            ),
            "accept"
        )?;

        SctpAssociation::from_accepted(assoc_fd, self.ppid, &self.logger)
    }
}

impl Drop for SctpListener {
    fn drop(&mut self) {
        unsafe { libc::close(self.fd) };
    }
}
