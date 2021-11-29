use super::Message;
//use crate::sctp::sctp_c_bindings::socket;
use crate::sctp::sctp_c_bindings;
use crate::sctp::sctp_c_bindings::{SCTP_NODELAY, SCTP_RECVRCVINFO, SOL_SCTP};
use async_io::Async;
use async_net::AsyncToSocketAddrs;
use libc::{connect, setsockopt, socket};
use libc::{AF_INET, IPPROTO_SCTP, SOCK_STREAM};
use os_socketaddr::OsSocketAddr;
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

impl SctpAssociation {
    // See https://docs.rs/async-net/1.6.1/async_net/struct.TcpStream.html
    pub async fn establish<A: AsyncToSocketAddrs>(addr: A) -> io::Result<SctpAssociation> {
        // Set up assocation as client
        let addr = async_net::resolve(addr).await.map(|vec| vec[0])?;
        let addr: OsSocketAddr = addr.into();

        let connected_socket_fd: i32 = unsafe {
            let s = socket(AF_INET, SOCK_STREAM, IPPROTO_SCTP);
            // TODO error handling

            // Disable nagling and enable SCTP_RCVINFO.
            // RFC6458, 8.1.29 - This option expects an integer boolean flag, where a non-zero value
            // turns on the option, and a zero value turns off the option.
            let enabled: libc::c_int = 1;
            let enabled = &enabled as *const _ as *const libc::c_void;
            let enabled_len = std::mem::size_of::<libc::c_int>() as libc::socklen_t;

            // let client_sock = SctpSocket::new(raw_addr.family(), SOCK_STREAM).unwrap();
            // client_sock
            //     //.setsockopt(sctp_sys::IPPROTO_SCTP, sctp_sys::SCTP_EVENTS, &events)

            println!("NODELAY setsockopt {:?}", s);
            if setsockopt(
                s,
                SOL_SCTP as libc::c_int,
                SCTP_NODELAY as libc::c_int,
                enabled,
                enabled_len,
            ) < 0
            {
                Err(Error::last_os_error())
            } else {
                Ok(())
            }?;

            println!("SCTP_RECVRCVINFO setsockopt {:?}", s);
            if setsockopt(
                s,
                SOL_SCTP as libc::c_int,
                SCTP_RECVRCVINFO as libc::c_int,
                enabled,
                enabled_len,
            ) < 0
            {
                Err(Error::last_os_error())
            } else {
                Ok(())
            }?;

            println!("Do connect");

            // TODO error handling
            // TODO nonblocking connect
            if connect(s, addr.as_ptr(), addr.len()) < 0 {
                Err(Error::last_os_error())
            } else {
                Ok(s)
            }
        }?;

        //let fd = Async::new(connected_socket_fd)?;

        // Put the fd into non-blocking mode.  This ought to be moved before the connect.

        Ok(SctpAssociation {
            fd: connected_socket_fd,
        })
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

        let message = unsafe {
            let bytes_received = libc::recvmsg(self.fd, &mut msghdr, 0);
            println!("Recvmsg returned {}", bytes_received);
            if bytes_received >= 0 {
                message.resize(bytes_received as usize, 0);
                Ok(message)
            } else {
                Err(Error::last_os_error())
            }
        }?;

        Ok(message)
    }
}
