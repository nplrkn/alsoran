use super::rrc::*;
use asn1_per::*;

#[test]
fn test_rrc_setup_container() -> Result<(), PerCodecError> {
    let hex = "100000000067";
    let bytes = hex::decode(hex).unwrap();

    let decoded_message = UlCcchMessage::from_bytes(&bytes)?;
    println!("Yay {:?}", decoded_message);

    Ok(())
}
