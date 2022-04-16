use super::ies::*;
use super::pdu::*;
use super::top_pdu::*;
use asn1_codecs::aper::*;
use bitvec::prelude::*;

#[test]
fn test_ng_setup_coding() -> Result<(), AperCodecError> {
    let plmn_identity = PlmnIdentity(vec![0x02, 0xf8, 0x39]);
    let _ng_setup = NgSetupRequest {
        global_ran_node_id: GlobalRanNodeId::GlobalGnbId(GlobalGnbId {
            plmn_identity: plmn_identity.clone(),
            gnb_id: GnbId::GnbId(bitvec![Msb0, u8; 0x00, 0x01, 0x02]),
        }),
        ran_node_name: Some(RanNodeName("free5GC".to_string())),
        supported_ta_list: SupportedTaList(vec![SupportedTaItem {
            tac: Tac(vec![0, 0, 1]),
            broadcast_plmn_list: BroadcastPlmnList(vec![BroadcastPlmnItem {
                plmn_identity: plmn_identity.clone(),
                tai_slice_support_list: SliceSupportList(vec![SliceSupportItem {
                    s_nssai: SNssai {
                        sst: Sst(vec![0x01]),
                        sd: Some(Sd(vec![1, 2, 3])),
                    },
                }]),
            }]),
        }]),
        default_paging_drx: PagingDrx::V128,
        ue_retention_information: None,
        nb_iot_default_paging_drx: None,
        extended_ran_node_name: None,
    };

    // This starts at the open type encoding of the NG Setup initiating message.

    let input_hex = "35000004001b00080002f83910000102005240090300667265653567630066001000000000010002f839000010080102030015400140";

    let bytes = hex::decode(input_hex).unwrap();
    let mut data = AperCodecData::from_slice(&bytes);
    let ng_setup_2 = NgSetupRequest::decode(&mut data)?;

    let mut encoded = AperCodecData::new();
    ng_setup_2.encode(&mut encoded)?;
    let output_hex = hex::encode(encoded.into_bytes());
    assert_eq!(input_hex, output_hex);

    Ok(())
}

#[test]
fn test_ngap_pdu_coding() -> Result<(), AperCodecError> {
    let input_hex = "00150035000004001b00080002f83910000102005240090300667265653567630066001000000000010002f839000010080102030015400140";
    let bytes = hex::decode(input_hex).unwrap();
    let mut data = AperCodecData::from_slice(&bytes);
    let ngap_pdu = NgapPdu::decode(&mut data)?;
    println!("Yay {:?}", ngap_pdu);

    Ok(())
}
