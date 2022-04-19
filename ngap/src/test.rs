use super::ies::*;
use super::pdu::*;
use super::top_pdu::*;
use asn1_codecs::aper::*;
use bitvec::prelude::*;

fn make_ng_setup() -> NgSetupRequest {
    let plmn_identity = PlmnIdentity(vec![0x02, 0xf8, 0x39]);
    NgSetupRequest {
        global_ran_node_id: GlobalRanNodeId::GlobalGnbId(GlobalGnbId {
            plmn_identity: plmn_identity.clone(),
            gnb_id: GnbId::GnbId(vec![0x00, 0x01, 0x02].view_bits::<Msb0>().into()),
        }),
        ran_node_name: Some(RanNodeName("free5gc".to_string())),
        supported_ta_list: SupportedTaList(vec![SupportedTaItem {
            tac: Tac(vec![0, 0, 1]),
            broadcast_plmn_list: BroadcastPlmnList(vec![BroadcastPlmnItem {
                plmn_identity: plmn_identity,
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
    }
}

#[test]
fn test_ngap_pdu_coding() -> Result<(), AperCodecError> {
    let ng_setup = make_ng_setup();
    let ngap_pdu = NgapPdu::InitiatingMessage(InitiatingMessage::NgSetupRequest(ng_setup));
    let mut encoded = AperCodecData::new();
    ngap_pdu.encode(&mut encoded)?;
    let output_hex = hex::encode(encoded.into_bytes());

    let reference = "00150035000004001b00080002f83910000102005240090300667265653567630066001000000000010002f839000010080102030015400140";
    assert_eq!(reference, output_hex);

    let bytes = hex::decode(reference).unwrap();
    let mut data = AperCodecData::from_slice(&bytes);
    let ngap_pdu = NgapPdu::decode(&mut data)?;
    let mut encoded = AperCodecData::new();
    ngap_pdu.encode(&mut encoded)?;
    let output_hex = hex::encode(encoded.into_bytes());
    assert_eq!(reference, output_hex);

    Ok(())
}
