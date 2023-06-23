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
        info!(logger, "Serving GTP-U on {transport_address}");
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

    pub async fn send_f1u_data_packet(
        &self,
        remote_gtpu_ip: IpAddr,
        gtp_teid: GtpTeid,
    ) -> Result<()> {
        let addr = SocketAddr::new(remote_gtpu_ip, GTPU_PORT);
        info!(
            self.logger,
            "Send F1U data packet to {remote_gtpu_ip}, {gtp_teid:?}"
        );

        let gtp_teid = gtp_teid.0;
        const GTP_MESSAGE_TYPE_GPU: u8 = 255; // TS29.281, table 6.1-1

        let packet = [
            // ---- GTP header ----
            0b001_1_0_0_0_0,      // version, PT, R, E, S, PN
            GTP_MESSAGE_TYPE_GPU, // message type
            0,
            31, // length of payload
            gtp_teid[0],
            gtp_teid[1],
            gtp_teid[2],
            gtp_teid[3], // TEID
            // ---- SDAP UPLINK DATA PDU ----
            0b1_0_000001, // D/C, R, QFI - see TS37.324
            // ---- PDCP Data PDU for DRB with 12 bit PDCP SN ----
            0b1_0_0_0_0000, // D/C, R,R,R, SN
            0b00000001,     // SN
            // ---- Inner IP header ----
            0b0100_0101, // version and header length
            0x00,        // differentiated services
            0x00,
            28, // total length
            0x00,
            0x00, // identification
            0x00,
            0x00, // flags + fragment offset,
            0x40, // TTL = 64,
            17,   // protocol = 17 = UDP,
            0x00,
            0x00, // IP header checksum
            1,
            1,
            1,
            1, // Source IP
            2,
            2,
            2,
            2, // Dest IP
            // ---- Inner UDP header ----
            0x80,
            0x00, // Source port = 32768
            0x80,
            0x01, // Dest port = 32769
            0x00,
            0x08, // Length = 8
            0x00,
            0x00, // Checksum
        ];

        let _bytes_sent = self.gtpu_socket.send_to(&packet, addr).await?;
        Ok(())
    }

    pub async fn send_n3_data_packet(
        &self,
        remote_gtpu_ip: IpAddr,
        gtp_teid: GtpTeid,
    ) -> Result<()> {
        let addr = SocketAddr::new(remote_gtpu_ip, GTPU_PORT);
        info!(
            self.logger,
            "Send N3 data packet to {remote_gtpu_ip}, {gtp_teid:?}"
        );

        let gtp_teid = gtp_teid.0;
        const GTP_MESSAGE_TYPE_GPU: u8 = 255; // TS29.281, table 6.1-1

        let packet = [
            // ---- GTP header ----
            0b001_1_0_1_0_0,      // version=1, PT=1, R, E=1, S, PN
            GTP_MESSAGE_TYPE_GPU, // message type
            0,
            36, // length of payload including extension headers
            gtp_teid[0],
            gtp_teid[1],
            gtp_teid[2],
            gtp_teid[3], // TEID
            0,
            0,          // Sequence number
            0,          // N-PDU number
            0b10000101, // next extension = PDU Session Container - see TS29.281, figure 5.2.1-3
            // ---- PDU Session Container ----
            1,              // length of this container / 4 = 4 bytes
            0b0000_0_0_0_0, // PDU type = DL PDU SESSION INFORMATION, QMP, SNP, MSNP, Spare
            0b0_0_000001,   // PPP, RQI, QFI=1,
            0,              // next extension type = none
            // ---- Inner IP header ----
            0b0100_0101, // version and header length
            0x00,        // differentiated services
            0x00,
            28, // total length
            0x00,
            0x00, // identification
            0x00,
            0x00, // flags + fragment offset,
            0x40, // TTL = 64,
            17,   // protocol = 17 = UDP,
            0x00,
            0x00, // IP header checksum
            2,
            2,
            2,
            2, // Source IP
            1,
            1,
            1,
            1, // Dest IP
            // ---- Inner UDP header ----
            0x80,
            0x01, // Source port = 32769
            0x80,
            0x00, // Dest port = 32768
            0x00,
            0x08, // Length = 0
            0x00,
            0x00, // Checksum
        ];

        let _bytes_sent = self.gtpu_socket.send_to(&packet, addr).await?;
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
