use std::sync::Arc;

use anyhow::Result;
use async_net::{IpAddr, SocketAddr, UdpSocket};
use async_std::{
    sync::Mutex,
    task::{self, JoinHandle},
};
use slog::{debug, Logger};
use xxap::{GtpTeid, GtpTunnel};

const GTPU_PORT: u16 = 2152; // TS29.281

#[derive(Clone)]
pub struct PacketProcessor {
    forwarding_table: Arc<Mutex<ForwardingTable>>,
    _forwarding_task: Arc<JoinHandle<()>>,
}

// The rule for forwarding a packet.
#[derive(Clone)]
pub struct ForwardingContext {
    pub session_1_downlink: Option<ForwardingAction>,
    pub session_1_uplink: Option<ForwardingAction>,
}

#[derive(Clone)]
pub struct ForwardingAction {
    pub remote_tunnel_info: GtpTunnel,
}

//pub type ForwardingRule = (u32, ForwardingAction);

// Forwarding table
struct ForwardingTable(Vec<ForwardingContext>);

// TODO - dynamically sizeable (or at least... bigger than 256 entries)
const CAPACITY_BITS: usize = 8;
const CAPACITY: usize = 1 << CAPACITY_BITS;
const CAPACITY_MASK: u32 = 0xff;

impl PacketProcessor {
    pub async fn new(local_ip: IpAddr, logger: Logger) -> Result<Self> {
        let transport_address = SocketAddr::new(local_ip, GTPU_PORT);
        let gtpu_socket = UdpSocket::bind(transport_address).await?;
        // See set_reuse_address in socket2 create.  May be necessary for HA.

        let forwarding_table = Arc::new(Mutex::new(ForwardingTable(vec![
            ForwardingContext {
                session_1_downlink: None,
                session_1_uplink: None
            };
            CAPACITY
        ])));

        let _forwarding_task =
            Arc::new(start_forwarding(gtpu_socket, forwarding_table.clone(), logger).await);

        Ok(PacketProcessor {
            forwarding_table,
            _forwarding_task,
        })
    }

    pub async fn set_forwarding_action(
        &self,
        gtp_teid: GtpTeid,
        action: ForwardingAction,
        logger: &Logger,
    ) {
        let gtp_teid_u32 = u32::from_be_bytes(gtp_teid.0);
        let key = ((gtp_teid_u32 >> 1) & CAPACITY_MASK) as usize;
        let context = &mut self.forwarding_table.lock().await.0[key];
        if (gtp_teid_u32 & 1) == 1 {
            debug!(
                logger,
                "Install downlink forwarding action {:?}->{}/{:?}",
                gtp_teid.0,
                action
                    .remote_tunnel_info
                    .transport_layer_address
                    .to_string(),
                action.remote_tunnel_info.gtp_teid.0
            );
            context.session_1_downlink = Some(action);
        } else {
            debug!(
                logger,
                "Install uplink forwarding action {:?}->{}/{:?}",
                gtp_teid.0,
                action
                    .remote_tunnel_info
                    .transport_layer_address
                    .to_string(),
                action.remote_tunnel_info.gtp_teid.0
            );
            context.session_1_uplink = Some(action);
        }
    }
}

async fn start_forwarding(
    gtpu_socket: UdpSocket,
    forwarding_table: Arc<Mutex<ForwardingTable>>,
    logger: Logger,
) -> JoinHandle<()> {
    task::spawn(async move {
        let mut buf = [0; 2000];
        loop {
            let Ok((n, _peer)) = gtpu_socket.recv_from(&mut buf).await else {
                break;
            };

            if n < GTP_HEADER_MIN_SIZE {
                // TODO - update stat
                continue;
            }

            let gtp_header = parse_gtp(&buf[0..GTP_HEADER_MIN_SIZE]);
            let gtp_teid = gtp_header.teid;
            let key = ((gtp_teid >> 1) & CAPACITY_MASK) as usize;
            let downlink = (gtp_teid & 1) == 1;

            debug!(
                logger,
                "Rx data packet TEID {:?}",
                gtp_header.teid.to_be_bytes()
            );

            let context = &forwarding_table.lock().await.0[key];

            // LSB clear for uplink; set for downlink
            let action = if downlink {
                &context.session_1_downlink
            } else {
                &context.session_1_uplink
            };
            let Some(action) = action else {
                continue;   // TODO update stat
            };

            let dest_ip: IpAddr = action
                .remote_tunnel_info
                .transport_layer_address
                .clone()
                .try_into()
                .unwrap(); // TODO don't unwrap
            let dest_sock_addr = SocketAddr::new(dest_ip, GTPU_PORT);

            debug!(
                logger,
                "Tx data packet TEID {:?}, {dest_sock_addr}", &action.remote_tunnel_info.gtp_teid.0
            );

            overwrite_teid(&mut buf, &action.remote_tunnel_info.gtp_teid.0);

            if downlink {
                replace_n3_with_f1_headers(&mut buf);
            } else {
                replace_f1_with_n3_headers(&mut buf);
            }

            match gtpu_socket.send_to(&buf[..n], dest_sock_addr).await {
                Ok(_bytes_sent) => (), // TODO update stat
                Err(_e) => (),         // TODO update stat
            }
        }
    })
}

const GTP_MESSAGE_TYPE_GPU: u8 = 255; // TS29.281, table 6.1-1

fn replace_n3_with_f1_headers(buf: &mut [u8; 2000], offset: &mut usize, gtp_teid: [u8; 4]) {
    // On the N3 side, we are expecting
    // - a 12-byte GTP header - making the GTP header 12 bytes
    // - a 4-byte PDU session container
    // ...meaning that the inner packet is at offset 16.
    // On the F1 side, we have
    // - a minimal 8-byte GTP header
    // - a 1-byte SDAP header
    // - a 2-byte PDCP header
    // ...meaning that the inner packet is at offset 11 and the packet is 5
    // bytes shorter.

    // TODO: read the inbound head and PDU session container
    let buf = &buf[*offset..];
    let orig_length = u16::from_be_bytes([buf[2], buf[3]]);
    let new_length = (orig_length - 5).to_be_bytes();

    // Advance offset and write in new header
    *offset += 5;
    let buf = &buf[5..];

    // Rebuild the header, ovewriting the received data.
    buf[0] = 0b001_1_0_0_0_0;
    buf[1] = GTP_MESSAGE_TYPE_GPU;
    buf[2] = new_length[0];
    buf[3] = new_length[1];
    buf[4] = gtp_teid[0];
    buf[5] = gtp_teid[1];
    buf[6] = gtp_teid[2];
    buf[7] = gtp_teid[3];

    // ---- SDAP DOWNLINK DATA PDU ----
    buf[8] = 0b0_0_000001; // RDI, RQI, QFI - see TS37.324

    // ---- PDCP Data PDU for DRB with 12 bit PDCP SN ----
    // TODO: handle PDCP sequence number correctly
    buf[9] = 0b1_0_0_0_0000; // D/C, R,R,R, SN
    buf[10] = 0b00000001; // SN
}

fn replace_f1_with_n3_headers(buf: &mut [u8; 2000]) {}

// From TS 29.281, 5.1
struct GtpHeader {
    _octet1: u8,
    _message_type: u8,
    _length: u16,
    teid: u32,
}
const GTP_HEADER_MIN_SIZE: usize = 8;
fn parse_gtp(packet: &[u8]) -> GtpHeader {
    let octet1 = packet[0];
    let message_type = packet[1];
    let length = ((packet[2] as u16) << 8) + packet[3] as u16;
    let teid = ((packet[4] as u32) << 24)
        + ((packet[5] as u32) << 16)
        + ((packet[6] as u32) << 8)
        + (packet[7] as u32);
    GtpHeader {
        _octet1: octet1,
        _message_type: message_type,
        _length: length,
        teid,
    }
}

fn overwrite_teid(packet: &mut [u8], new_teid: &[u8; 4]) {
    packet[4..8].copy_from_slice(new_teid)
}
