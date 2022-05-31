use anyhow::{anyhow, Result};

pub struct PdcpPdu(pub Vec<u8>);

impl PdcpPdu {
    pub fn encode(inner: &[u8]) -> Self {
        let mut pdcp_pdu = vec![0u8, 0u8]; // 4 bits reserved, 12 bits of sequence numbers
        pdcp_pdu.extend(inner);
        pdcp_pdu.extend([0, 0, 0, 0]); // 4 bytes of MAC
        Self(pdcp_pdu)
    }

    pub fn view_inner(&self) -> Result<&[u8]> {
        if self.0.len() < 6 {
            return Err(anyhow!("Too short for PDCP PDU".to_string()));
        }
        Ok(&self.0[2..self.0.len() - 4])
    }

    pub fn bytes(self) -> Vec<u8> {
        self.0
    }
}

// /// Wrap an inner packet in a PDCP data PDU
// pub fn into_data_pdu(inner: &[u8]) -> Vec<u8> {
//     let mut pdcp_pdu = vec![0u8, 0u8]; // 4 bits reserved, 12 bits of sequence numbers
//     pdcp_pdu.extend(inner);
//     pdcp_pdu.extend([0, 0, 0, 0]); // 4 bytes of MAC
//     pdcp_pdu
// }

// /// Get the inner packet from a PDCP data PDU.
