use anyhow::{anyhow, Result};

/// Wrap an inner packet in a PDCP data PDU
pub fn into_data_pdu(inner: &[u8]) -> Vec<u8> {
    let mut pdcp_pdu = vec![0u8, 0u8]; // 4 bits reserved, 12 bits of sequence numbers
    pdcp_pdu.extend(inner);
    pdcp_pdu.extend([0, 0, 0, 0]); // 4 bytes of MAC
    pdcp_pdu
}

/// Get the inner packet from a PDCP data PDU.
pub fn view_inner(pdcp_pdu: &[u8]) -> Result<&[u8]> {
    if pdcp_pdu.len() < 6 {
        return Err(anyhow!("Too short for PDCP PDU".to_string()));
    }
    Ok(&pdcp_pdu[2..pdcp_pdu.len() - 4])
}
