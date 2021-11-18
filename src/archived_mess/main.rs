mod f1_transport_cu;
mod f1ap_procedures;
mod models;
mod ngap_transport_cu;
mod ngap_receiver;

use f1_transport_cu::F1TransportCu;
use ngap_transport_cu::NGAPTransportCu;
use std::net::{IpAddr, Ipv4Addr};

struct Config {
    amf_sctp_initial_ip_address: IpAddr,
}

use std::sync::Arc;

struct GNBCU {
    f1_transport: F1TransportCu,
    ngap_transport: NGAPTransportCu,
}

impl GNBCU {
    pub async fn new(c: Config) -> Result<GNBCU> {
        // Start F1 transport.
        const F1AP_SCTP_DESTINATION_PORT: u16 = 38472;
        const NGAP_SCTP_DESTINATION_PORT: u16 = 38412;
        let f1_listen_address = (Ipv4Addr::UNSPECIFIED, F1AP_SCTP_DESTINATION_PORT);
        let f1_transport = F1TransportCu::new(f1_listen_address)?;

        // Start NGAP transport.
        const NGAP_SCTP_DESTINATION_PORT: u16 = 38412;
        let ngap_connect_address = (c.amf_sctp_initial_ip_address, NGAP_SCTP_DESTINATION_PORT);
        let ngap_transport = NGAPTransportCu::new(ngap_connect_address)?;

        let gnbcu = GNBCU {
            f1_transport,
            ngap_transport,
        };
        gnbcu
            .f1_transport
            .set_handler(Arc::downgrade(Arc::new(&gnbcu)));
        gnbcu
            .ngap_transport
            .set_handler(Arc::downgrade(Arc::new(&gnbcu)));

        Ok(gnbcu)
    }
}

impl f1ap_procedures::DuInitiatedOperations for GNBCU {}

#[async_std::main]
async fn main() {
    // Create a node configuration.
    let config = Config {
        amf_sctp_initial_ip_address: Ipv4Addr::LOCALHOST.into(),
    };
}

// TS38.412, 7
// The Payload Protocol Identifier (ppid) assigned by IANA to be used by SCTP for the application layer protocol NGAP
// is 60, and 66 for DTLS over SCTP (IETF RFC 6083 [8]).
const NGAP_SCTP_PPID: u16 = 60;

mod rust_sctp;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     extern crate async_std;
//     //use rust_sctp::{SctpListener, SctpStream};
//     use async_io::Async;
//     use libc::{
//         accept, AF_INET, AF_INET6, SOCK_STREAM, SOL_SOCKET, SO_RCVBUF, SO_RCVTIMEO, SO_SNDBUF,
//         SO_SNDTIMEO,
//     };
//     use rust_sctp::sctpsock::*;
//     use std::io::Error;
//     use std::net::SocketAddr;
//     use std::os::unix::io::{AsRawFd, FromRawFd};

//     #[derive(Debug, Copy, Clone, Default)]
//     #[repr(C)]
//     pub struct sctp_event_subscribe {
//         pub sctp_data_io_event: u8,
//         pub sctp_association_event: u8,
//         pub sctp_address_event: u8,
//         pub sctp_send_failure_event: u8,
//         pub sctp_peer_error_event: u8,
//         pub sctp_shutdown_event: u8,
//         pub sctp_partial_delivery_event: u8,
//         pub sctp_adaptation_layer_event: u8,
//         pub sctp_authentication_event: u8,
//         pub sctp_sender_dry_event: u8,
//         pub sctp_stream_reset_event: u8,
//     }

//     #[derive(Debug, Copy, Clone, Default)]
//     #[repr(C)]
//     struct sctp_rcvinfo {
//         pub rcv_sid: u16,
//         pub rcv_ssn: u16,
//         pub rcv_flags: u16,
//         pub rcv_ppid: u32,
//         pub rcv_tsn: u32,
//         pub rcv_cumtsn: u32,
//         pub rcv_context: u32,
//         pub rcv_assoc_id: sctp_sys::sctp_assoc_t,
//     }

//     #[async_std::test]
//     async fn fix_sctp() {
//         let address_port: (IpAddr, u16) = (Ipv4Addr::LOCALHOST.into(), 10000);

//         // Set up listen socket
//         let raw_addr = SocketAddr::from_addr(&address_port).unwrap();
//         let listen_sock = SctpSocket::new(raw_addr.family(), SOCK_STREAM).unwrap();
//         listen_sock.bind(raw_addr).unwrap();
//         listen_sock.listen(-1).unwrap();

//         // // Initialize the option structure that allows us to get stream info on receive
//         // let events = sctp_event_subscribe {
//         //     sctp_data_io_event: 1,
//         //     ..sctp_event_subscribe::default()
//         // };

//         // Set up assocation as client
//         let client_sock = SctpSocket::new(raw_addr.family(), SOCK_STREAM).unwrap();
//         client_sock
//             //.setsockopt(sctp_sys::IPPROTO_SCTP, sctp_sys::SCTP_EVENTS, &events)
//             .setsockopt(sctp_sys::IPPROTO_SCTP, sctp_sys::SCTP_RECVRCVINFO, &1)
//             .unwrap();
//         client_sock.connect(raw_addr).unwrap();

//         // Set up association as server
//         let server_sock = unsafe {
//             let fd = accept(
//                 listen_sock.as_raw_fd(),
//                 std::ptr::null_mut(),
//                 std::ptr::null_mut(),
//             );
//             SctpSocket::from_raw_fd(fd)
//         };
//         server_sock
//             .setsockopt(sctp_sys::IPPROTO_SCTP, sctp_sys::SCTP_RECVRCVINFO, &1)
//             .unwrap();

//         // Transmit a message from client and receive at server
//         let msg = "Hello server".as_bytes();
//         let mut buffer: [u8; 255] = [0; 255];
//         let bytes_rcvd;
//         let mut info: sctp_sys::sctp_sndrcvinfo = unsafe { std::mem::zeroed() };
//         let mut info: sctp_sys::sctp_sndrcvinfo = unsafe { std::mem::zeroed() };
//         unsafe {
//             sctp_sys::sctp_sendmsg(
//                 client_sock.as_raw_fd(),
//                 msg.as_ptr() as *const libc::c_void,
//                 msg.len() as libc::size_t,
//                 std::ptr::null_mut(),
//                 53, // ppid
//                 0,
//                 0,
//                 5 as u16, // stream ID
//                 100,      // ttl
//                 0,
//             );

//             // bytes_rcvd = sctp_sys::sctp_recvmsg(
//             //     server_sock.as_raw_fd(),
//             //     buffer.as_mut_ptr() as *mut libc::c_void,
//             //     255 as libc::size_t, // buffer size
//             //     std::ptr::null_mut(),
//             //     std::ptr::null_mut(),
//             //     &mut info,
//             //     std::ptr::null_mut(),
//             // );

//             let mut info = sctp_sys::sctp_rcvinfo;
//             let mut infotype : uint;

//             bytes_rcvd = sctp_sys::recvv(server_sock.as_raw_fd(),
//                                           std::ptr::null_mut(),  // iov
//                                           0,  // iovlen
//                                           std::ptr::null_mut(),  // from
//                                           std::ptr::null_mut(), // fromlen
//                                           &mut info,
//                                           mem::size_of::<sctp_sys::sctp_rcvinfo>(),
//                                           &mut infotype,
//                                           std::ptr::null_mut());

//         )
//         }

//         println!(
//             "Received {} with error {} stream info {:?}",
//             bytes_rcvd,
//             Error::last_os_error(),
//             info
//         );

//         // Spawn an async connection handler.
//         let task = async_std::task::spawn(async {
//             let assoc = Async::new(server_sock).unwrap();
//             assoc.readable().await.unwrap();
//             let mut buffer: [u8; 255] = [0; 255];
//             let mut info: sctp_sys::sctp_sndrcvinfo = unsafe { std::mem::zeroed() };

//             let bytes_rcvd = unsafe {
//                 sctp_sys::sctp_recvmsg(
//                     assoc.as_raw_fd(),
//                     buffer.as_mut_ptr() as *mut libc::c_void,
//                     255 as libc::size_t, // buffer size
//                     std::ptr::null_mut(),
//                     std::ptr::null_mut(),
//                     &mut info,
//                     std::ptr::null_mut(),
//                 )
//             };
//             println!(
//                 "Received {} with error {} stream info {:?}",
//                 bytes_rcvd,
//                 Error::last_os_error(),
//                 info
//             );
//         });

//         let msg = "Me again".as_bytes();
//         unsafe {
//             sctp_sys::sctp_sendmsg(
//                 client_sock.as_raw_fd(),
//                 msg.as_ptr() as *const libc::c_void,
//                 msg.len() as libc::size_t,
//                 std::ptr::null_mut(),
//                 53, // ppid
//                 0,
//                 0,
//                 5 as u16, // stream ID
//                 100,      // ttl
//                 0,
//             );
//         }

//         task.await;
//     }

//     // #[async_std::test]
//     // async fn sctp_works() {
//     //     // The following code works, but the stream ID is not being passed correctly.
//     //     // And it is not properly async, so probably working by luck.
//     //     let address_port: (IpAddr, u16) = (Ipv4Addr::LOCALHOST.into(), 10000);
//     //     println!("listening");
//     //     let listener = SctpListener::bind(address_port.clone()).unwrap();

//     //     let task = async_std::task::spawn(async move {
//     //         println!("connecting");
//     //         let stream = SctpStream::connect(address_port).unwrap();
//     //         println!("got it!");
//     //         stream.sendmsg("hello on stream 5".as_bytes(), 9).unwrap();
//     //     });

//     //     let (stream, _) = listener.accept().unwrap();
//     //     task.await;
//     //     let mut buffer: [u8; 255] = [0; 255];
//     //     let (bytes_read, stream_id) = stream.recvmsg(&mut buffer).unwrap();
//     //     println!(
//     //         "read {} bytes from stream {}: {:?}",
//     //         bytes_read, stream_id, buffer
//     //     )
//     // }
// }
