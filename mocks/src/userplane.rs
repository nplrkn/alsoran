use std::time::Duration;

use anyhow::Result;
use async_net::{IpAddr, SocketAddr, UdpSocket};
use async_std::future;
use slog::{debug, info, Logger};
use xxap::GtpTeid;

const GTPU_PORT: u16 = 2152; // TS29.281

pub struct MockUserplane {
    local_ip: IpAddr,
    gtpu_socket: UdpSocket,
    logger: Logger,
}

impl MockUserplane {
    pub async fn new(local_ip: &str, logger: Logger) -> Result<Self> {
        let transport_address = format!("{}:{}", local_ip, GTPU_PORT);
        info!(logger, "Binding GTP-U socket to {transport_address}");
        let gtpu_socket = UdpSocket::bind(transport_address).await?;
        Ok(MockUserplane {
            local_ip: local_ip.parse()?,
            gtpu_socket,
            logger,
        })
    }

    pub fn local_ip(&self) -> &IpAddr {
        &self.local_ip
    }

    pub async fn send_data_packet(&self, remote_gtpu_ip: IpAddr, gtp_teid: GtpTeid) -> Result<()> {
        let addr = SocketAddr::new(remote_gtpu_ip, GTPU_PORT);
        info!(
            self.logger,
            "Send data packet to {remote_gtpu_ip}, {gtp_teid:?}"
        );

        let gtp_teid = gtp_teid.0;
        const GTP_MESSAGE_TYPE_GPU: u8 = 255; // TS29.281, table 6.1-1

        let gtp_packet = [
            0b001_1_0_0_0, // version, protocol type, extension, sequence number, N-PDU number flag
            GTP_MESSAGE_TYPE_GPU, // message type
            0,
            8, // length of payload (8 bytes)
            gtp_teid[0],
            gtp_teid[1],
            gtp_teid[2],
            gtp_teid[3], // TEID
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8, // payload
        ];

        let _bytes_sent = self.gtpu_socket.send_to(&gtp_packet, addr).await?;
        Ok(())
    }

    pub async fn recv_data_packet(&self, _gtp_teid: &GtpTeid) -> Result<()> {
        debug!(self.logger, "Wait for data packet");
        let mut packet = Vec::with_capacity(2000);
        let future_result = self.gtpu_socket.recv_from(packet.as_mut_slice());
        let (_bytes_received, _source_address) =
            future::timeout(Duration::from_secs(1), future_result).await??;
        Ok(())
    }
}
