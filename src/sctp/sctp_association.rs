use super::Message;
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
    pub async fn establish<A: AsyncToSocketAddrs>(
        addr: A,
        ppid: u32,
        logger: &Logger,
    ) -> io::Result<SctpAssociation> {
        // Get a socket and immediately wrap it in an SctpAssociation to ensure it gets closed
        // properly in the drop function if something fails later in this function.
        let fd = unsafe { socket(AF_INET, SOCK_STREAM, IPPROTO_SCTP) };
        let assoc = if fd < 0 {
            let e = Error::last_os_error();
            error!(logger, "Failed to get SCTP socket - {}", e);
            Err(e)
        } else {
            Ok(SctpAssociation { fd, ppid })
        }?;

        // Set up sock opts
        set_sock_opts(assoc.fd, logger)?;

        // Connect
        // TODO nonblocking
        let addr = async_net::resolve(addr).await.map(|vec| vec[0])?;
        let addr: OsSocketAddr = addr.into();
        if unsafe { connect(assoc.fd, addr.as_ptr(), addr.len()) } < 0 {
            let e = Error::last_os_error();
            error!(logger, "Failed SCTP connect to {:?} - {}", addr, e);
            Err(e)
        } else {
            Ok(())
        }?;

        Ok(assoc)
    }

    pub async fn recv_msg(&self) -> io::Result<Message> {
        // Wait for the socket to become readable
        Async::new(self.fd)?.readable().await?;

        let mut message: Message = vec![0; 1500];
        let msg_iov = &mut libc::iovec {
            iov_base: message.as_mut_ptr() as _,
            iov_len: message.len(),
        };

        let mut msghdr = make_msghdr(&mut sctp_c_bindings::sctp_rcvinfo::default(), msg_iov);
        let bytes_received = unsafe { libc::recvmsg(self.fd, &mut msghdr, 0) };
        if bytes_received >= 0 {
            message.resize(bytes_received as _, 0);
            Ok(message)
        } else {
            Err(Error::last_os_error())
        }
    }

    pub async fn send_msg(&self, mut message: Message) -> io::Result<()> {
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
            pub snd_assoc_id: sctp_c_bindings::sctp_assoc_t,
        }

        let mut sndinfo = Sndinfo {
            cmsg_len: std::mem::size_of::<Sndinfo>(),
            cmsg_level: IPPROTO_SCTP,
            cmsg_type: sctp_c_bindings::sctp_cmsg_type_SCTP_SNDINFO as _,
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

        let bytes_sent = unsafe { libc::sendmsg(self.fd, &msghdr, libc::MSG_DONTWAIT) };
        if bytes_sent == message.len() as _ {
            Ok(())
        } else if bytes_sent >= 0 {
            // TODO Back pressure partial send
            println!("Partial send {} bytes of {}", bytes_sent, message.len());
            unimplemented!();
        } else {
            Err(Error::last_os_error())
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
        msg_controllen: std::mem::size_of::<T>(),
        msg_flags: 0,
    }
}

// Disable nagling and enable SCTP_RCVINFO on a given socket.
fn set_sock_opts(fd: i32, logger: &Logger) -> io::Result<()> {
    // RFC6458, 8.1.29 - This option expects an integer boolean flag, where a non-zero value
    // turns on the option, and a zero value turns off the option.
    let enabled = &1 as *const _ as _;
    let enabled_len = std::mem::size_of::<libc::c_int>() as _;
    if unsafe { setsockopt(fd, SOL_SCTP as _, SCTP_NODELAY as _, enabled, enabled_len) } < 0 {
        warn!(
            logger,
            "Failed to set NODELAY socket option - {}",
            Error::last_os_error()
        );
    };

    if unsafe {
        setsockopt(
            fd,
            SOL_SCTP as _,
            SCTP_RECVRCVINFO as _,
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
