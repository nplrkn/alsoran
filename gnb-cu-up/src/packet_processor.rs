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

    pub async fn set_uplink_forwarding_action(
        &self,
        gtp_teid: GtpTeid,
        action: ForwardingAction,
        logger: &Logger,
    ) {
        let gtp_teid_u32 = u32::from_be_bytes(gtp_teid.0);
        let key = ((gtp_teid_u32 >> 1) & CAPACITY_MASK) as usize;
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
        self.forwarding_table.lock().await.0[key].session_1_uplink = Some(action);
    }

    pub async fn set_downlink_forwarding_action(
        &self,
        gtp_teid: GtpTeid,
        action: ForwardingAction,
        logger: &Logger,
    ) {
        let gtp_teid_u32 = u32::from_be_bytes(gtp_teid.0);
        let key = ((gtp_teid_u32 >> 1) & CAPACITY_MASK) as usize;
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
        self.forwarding_table.lock().await.0[key].session_1_downlink = Some(action);
    }

    pub async fn clear_forwarding_actions(&self, gtp_teid: GtpTeid) {
        let gtp_teid_u32 = u32::from_be_bytes(gtp_teid.0);
        let key = ((gtp_teid_u32 >> 1) & CAPACITY_MASK) as usize;
        let context = &mut self.forwarding_table.lock().await.0[key];
        context.session_1_downlink = None;
        context.session_1_uplink = None;
    }
}

const HEADROOM: usize = 8;
async fn start_forwarding(
    gtpu_socket: UdpSocket,
    forwarding_table: Arc<Mutex<ForwardingTable>>,
    logger: Logger,
) -> JoinHandle<()> {
    task::spawn(async move {
        let mut buf = [0; 2000];
        loop {
            let mut offset = HEADROOM;
            let Ok((n, _peer)) = gtpu_socket.recv_from(&mut buf[offset..2000]).await else {
                break;
            };

            if n < GTP_HEADER_MIN_SIZE {
                // TODO - update stat
                continue;
            }

            let gtp_header = parse_gtp(&buf[offset..offset + GTP_HEADER_MIN_SIZE]);
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

            if downlink {
                replace_n3_with_f1_headers(
                    &mut buf,
                    &mut offset,
                    &action.remote_tunnel_info.gtp_teid.0,
                );
            } else {
                replace_f1_with_n3_headers(
                    &mut buf,
                    &mut offset,
                    &action.remote_tunnel_info.gtp_teid.0,
                );
            }

            match gtpu_socket
                .send_to(&buf[offset..(n + HEADROOM)], dest_sock_addr)
                .await
            {
                Ok(_bytes_sent) => (), // TODO update stat
                Err(_e) => (),         // TODO update stat
            }
        }
    })
}

const GTP_MESSAGE_TYPE_GPU: u8 = 255; // TS29.281, table 6.1-1

fn replace_n3_with_f1_headers(buf: &mut [u8; 2000], offset: &mut usize, gtp_teid: &[u8; 4]) {
    // On the N3 side, there should be
    // - a 12-byte GTP header
    // - a 4-byte PDU session container
    // ...meaning that the inner packet is at offset 16.
    //
    // On the F1 side, we have
    // - an 8-byte GTP header
    // - a 1-byte SDAP header
    // - a 2-byte PDCP header
    // ...meaning that the inner packet is at offset 11 and the packet is 5
    // bytes shorter on tx.

    // TODO: read the inbound header and PDU session container
    let gtp_header = &mut buf[*offset..];
    let orig_length = u16::from_be_bytes([gtp_header[2], gtp_header[3]]);
    let new_length = (orig_length - 5).to_be_bytes();

    // Advance offset and write in new header
    *offset += 5;
    let buf = &mut buf[*offset..];

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

fn replace_f1_with_n3_headers(buf: &mut [u8; 2000], offset: &mut usize, gtp_teid: &[u8; 4]) {
    // The inverse of the case above - so we need to grow the packet by 5 bytes.

    let gtp_header = &mut buf[*offset..];
    let orig_length = u16::from_be_bytes([gtp_header[2], gtp_header[3]]);
    let new_length = (orig_length + 5).to_be_bytes();

    // TODO: check rather than assume the inbound headers are there

    // Rewind offset and write in new header
    *offset -= 5;
    let buf = &mut buf[*offset..];

    // Rebuild the header, ovewriting the received data.
    // ---- GTP header, TS29.281 ----
    buf[0] = 0b001_1_0_1_0_0; // version=1, PT=1, R, E=1, S, PN
    buf[1] = GTP_MESSAGE_TYPE_GPU;
    buf[2] = new_length[0];
    buf[3] = new_length[1];
    buf[4] = gtp_teid[0];
    buf[5] = gtp_teid[1];
    buf[6] = gtp_teid[2];
    buf[7] = gtp_teid[3];
    buf[8] = 0; // Sequence number
    buf[9] = 0; // Sequence number
    buf[10] = 0; // N-PDU number
    buf[11] = 0b10000101; // next extension = PDU Session Container - see TS29.281, figure 5.2.1-3

    // ---- PDU session container, TS38.415 ----
    buf[12] = 1; // length of PDU session container = 4 bytes
    buf[13] = 0b0001_0_0_0_0; // PDU type = UL PDU SESSION INFORMATION, QMP, DL delay, UL delay, SNP
    buf[14] = 0b0_0_000001; // N3 delay, new IE, QFI=1,
    buf[15] = 0; // next extension type = none
}

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
