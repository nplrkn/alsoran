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
        let mut buf = vec![0; 2000];
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

            debug!(
                logger,
                "Rx data packet TEID {:?}",
                gtp_header.teid.to_be_bytes()
            );

            let context = &forwarding_table.lock().await.0[key];

            // LSB clear for uplink; set for downlink
            let action = if (gtp_teid & 1) == 1 {
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

            match gtpu_socket.send_to(&buf[..n], dest_sock_addr).await {
                Ok(_bytes_sent) => (), // TODO update stat
                Err(_e) => (),         // TODO update stat
            }
        }
    })
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

fn overwrite_teid(packet: &mut [u8], new_teid: &[u8; 4]) {
    packet[4..8].copy_from_slice(new_teid)
}
