//! lib - wrap and unwrap PDCP packets

use anyhow::{ensure, Result};

pub struct PdcpPdu(pub Vec<u8>);

impl PdcpPdu {
    /// Encapsulate an inner packet in an outer PDCP packet.
    pub fn encode(inner: &[u8]) -> Self {
        let mut pdcp_pdu = vec![0u8, 0u8]; // 4 bits reserved, 12 bits of sequence numbers
        pdcp_pdu.extend(inner);
        pdcp_pdu.extend([0, 0, 0, 0]); // 4 bytes of MAC
        Self(pdcp_pdu)
    }

    /// View the inner packet in a PDCP packet.
    pub fn view_inner(&self) -> Result<&[u8]> {
        ensure!(self.0.len() >= 6, "Too short for PDCP PDU");
        Ok(&self.0[2..self.0.len() - 4])
    }
}

impl From<PdcpPdu> for Vec<u8> {
    fn from(p: PdcpPdu) -> Self {
        p.0
    }
}
