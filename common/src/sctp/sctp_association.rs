use super::sctp_bindings::*;
use super::sock_opt;
use super::try_io::try_io;
use super::Message;
use anyhow::{anyhow, Result};
use async_io::Async;
use io::Error;
use libc::{connect, socket, AF_INET, IPPROTO_SCTP, SOCK_STREAM};
use os_socketaddr::OsSocketAddr;
use slog::{warn, Logger};
use std::os::unix::io::{AsRawFd, RawFd};
use std::{io, mem};

/// An SCTP assocation.
// Cannot be Cloned since it is the owner of the fd.  Instead use Arc.
#[derive(Debug)]
pub struct SctpAssociation {
    fd: i32,
    ppid: u32,
}

impl AsRawFd for SctpAssociation {
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }
}

impl Drop for SctpAssociation {
    fn drop(&mut self) {
        unsafe { libc::close(self.fd) };
    }
}

impl SctpAssociation {
    // Establish an association as a client
    pub async fn establish(
        addr: OsSocketAddr,
        ppid: u32,
        logger: &Logger,
    ) -> Result<SctpAssociation> {
        // Get a socket and immediately wrap it in an SctpAssociation to ensure it gets closed
        // properly in the drop function if something fails later in this function.
        let fd = try_io!(socket(AF_INET, SOCK_STREAM, IPPROTO_SCTP), "socket")?;
        let assoc = SctpAssociation { fd, ppid };

        // Connect
        // TODO nonblocking
        try_io!(connect(assoc.fd, addr.as_ptr(), addr.len()), "connect")?;

        assoc.set_sock_opts(logger)?;

        Ok(assoc)
    }

    pub fn from_accepted(fd: i32, ppid: u32, logger: &Logger) -> Result<SctpAssociation> {
        let assoc = SctpAssociation { fd, ppid };
        assoc.set_sock_opts(logger)?;
        Ok(assoc)
    }

    fn set_sock_opts(&self, logger: &Logger) -> Result<()> {
        let fd = self.fd;

        sock_opt::enable_sctp_heartbeat(fd, 1000).unwrap_or_else(|e| {
            warn!(logger, "Carrying on without heartbeat - {}", e);
        });
        sock_opt::enable_sock_opt(fd, SCTP_NODELAY as _).unwrap_or_else(|e| {
            warn!(logger, "Carrying on without NODELAY - {}", e);
        });
        sock_opt::enable_sock_opt(fd, SCTP_RECVRCVINFO as _)?;
        Ok(())
    }

    pub async fn recv_msg(&self) -> Result<Message> {
        // Wait for the socket to become readable
        Async::new(self.fd)?.readable().await?;

        let mut message: Message = vec![0; 1500];
        let msg_iov = &mut libc::iovec {
            iov_base: message.as_mut_ptr() as _,
            iov_len: message.len(),
        };

        let mut msghdr = make_msghdr(&mut sctp_rcvinfo::default(), msg_iov);
        let bytes_received = try_io!(libc::recvmsg(self.fd, &mut msghdr, 0), "recvmsg")?;
        if bytes_received > 0 {
            message.resize(bytes_received as _, 0);
            Ok(message)
        } else {
            Err(anyhow!("Connection terminated"))
        }
    }

    pub async fn send_msg(&self, mut message: Message) -> Result<()> {
        // Wait for the socket to become writable
        //Async::new(self.fd)?.writable().await?;

        #[repr(C)]
        #[derive(Debug)]
        // A libc::cmsghdr glued onto a sctp_c_bindings::sctp_sndinfo.
        struct Sndinfo {
            pub cmsg_len: libc::size_t,
            pub cmsg_level: ::std::os::raw::c_int,
            pub cmsg_type: ::std::os::raw::c_int,
            pub snd_sid: libc::__u16,
            pub snd_flags: libc::__u16,
            pub snd_ppid: libc::__u32,
            pub snd_context: libc::__u32,
            pub snd_assoc_id: sctp_assoc_t,
        }

        let mut sndinfo = Sndinfo {
            cmsg_len: mem::size_of::<Sndinfo>(),
            cmsg_level: IPPROTO_SCTP,
            cmsg_type: sctp_cmsg_type_SCTP_SNDINFO as _,
            snd_sid: 1,
            snd_flags: 0,
            snd_ppid: self.ppid.to_be(),
            snd_context: 0,
            snd_assoc_id: 0,
        };

        let msg_iov = &mut libc::iovec {
            iov_base: message.as_mut_ptr() as _,
            iov_len: message.len(),
        };
        let msghdr = make_msghdr(&mut sndinfo, msg_iov);

        let bytes_sent = try_io!(
            libc::sendmsg(self.fd, &msghdr, libc::MSG_DONTWAIT),
            "sendmsg"
        )?;
        if bytes_sent == message.len() as isize {
            Ok(())
        } else {
            // TODO Back pressure partial send
            println!("Partial send {} bytes of {}", bytes_sent, message.len());
            unimplemented!();
        }
    }
}

fn make_msghdr<T>(msg_control: &mut T, msg_iov: &mut libc::iovec) -> libc::msghdr {
    libc::msghdr {
        msg_name: std::ptr::null_mut(),
        msg_namelen: 0,
        msg_iov,
        msg_iovlen: 1,
        msg_control: msg_control as *mut _ as _,
        msg_controllen: mem::size_of::<T>(),
        msg_flags: 0,
    }
}