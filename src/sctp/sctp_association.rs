use super::Message;
//use crate::sctp::sctp_c_bindings::socket;
use crate::sctp::sctp_c_bindings;
use crate::sctp::sctp_c_bindings::{SCTP_NODELAY, SCTP_RECVRCVINFO, SOL_SCTP};
use async_io::Async;
use async_net::AsyncToSocketAddrs;
use libc::{connect, setsockopt, socket};
use libc::{AF_INET, IPPROTO_SCTP, SOCK_STREAM};
use os_socketaddr::OsSocketAddr;
use slog::{error, warn, Logger};
use std::io;
use std::io::Error;
use std::os::unix::io::{AsRawFd, RawFd};

// An SCTP assocation.
#[derive(Debug, Clone)]
pub struct SctpAssociation {
    fd: i32,
}

impl AsRawFd for SctpAssociation {
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }
}

// Disable nagling and enable SCTP_RCVINFO on a given socket.
fn set_sock_opts(fd: i32, logger: &Logger) -> io::Result<()> {
    // RFC6458, 8.1.29 - This option expects an integer boolean flag, where a non-zero value
    // turns on the option, and a zero value turns off the option.
    let enabled = &1 as *const _ as *const _;
    let enabled_len = std::mem::size_of::<libc::c_int>() as libc::socklen_t;
    if unsafe {
        setsockopt(
            fd,
            SOL_SCTP as libc::c_int,
            SCTP_NODELAY as libc::c_int,
            enabled,
            enabled_len,
        )
    } < 0
    {
        warn!(
            logger,
            "Failed to set NODELAY socket option - {}",
            Error::last_os_error()
        );
    };

    if unsafe {
        setsockopt(
            fd,
            SOL_SCTP as libc::c_int,
            SCTP_RECVRCVINFO as libc::c_int,
            enabled,
            enabled_len,
        )
    } < 0
    {
        let e = Error::last_os_error();
        error!(
            logger,
            "Failed to set SCTP_RECVRCVINFO socket option - {}", e
        );
        Err(e)
    } else {
        Ok(())
    }
}

impl SctpAssociation {
    // Establish an association as a client
    pub async fn establish<A: AsyncToSocketAddrs>(
        addr: A,
        logger: &Logger,
    ) -> io::Result<SctpAssociation> {
        // Get a socket
        let fd = unsafe { socket(AF_INET, SOCK_STREAM, IPPROTO_SCTP) };

        // Set up sock opts
        set_sock_opts(fd, logger)?;

        // Connect
        // TODO nonblocking
        let addr = async_net::resolve(addr).await.map(|vec| vec[0])?;
        let addr: OsSocketAddr = addr.into();
        if unsafe { connect(fd, addr.as_ptr(), addr.len()) } < 0 {
            let e = Error::last_os_error();
            error!(logger, "Failed SCTP connect to {:?} - {}", addr, e);
            Err(e)
        } else {
            Ok(())
        }?;

        Ok(SctpAssociation { fd })
    }

    // pub async fn send(&self, _buf: &[u8], _stream_id: u32) -> Result<usize> {
    //     unimplemented!();
    // }

    pub async fn recv_msg(&self) -> io::Result<Message> {
        // Wait for the socket to become readable
        Async::new(self.fd)?.readable().await?;

        // Allocate message buffer
        let mut message: Message = vec![0; 1500];
        let msg_iov = &mut libc::iovec {
            iov_base: message.as_mut_ptr() as *mut libc::c_void,
            iov_len: message.len(),
        };

        // Ancillary data structure to receive the stream ID
        let msg_control =
            &mut sctp_c_bindings::sctp_rcvinfo::default() as *mut _ as *mut libc::c_void;

        // Set up structure to pass info into / get back from recvmsg.
        let mut msghdr = libc::msghdr {
            msg_name: std::ptr::null_mut(),
            msg_namelen: 0,
            msg_iov,
            msg_iovlen: 1, // elements in msg_iov
            msg_control,
            msg_controllen: std::mem::size_of::<sctp_c_bindings::sctp_rcvinfo>(),
            msg_flags: 0,
        };

        let bytes_received = unsafe { libc::recvmsg(self.fd, &mut msghdr, 0) };
        if bytes_received >= 0 {
            message.resize(bytes_received as usize, 0);
            Ok(message)
        } else {
            Err(Error::last_os_error())
        }
    }
}
