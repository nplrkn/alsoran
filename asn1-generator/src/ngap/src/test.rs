use super::ies::*;
use super::pdu::*;
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
    };

    // This starts at the open type encoding of the NG Setup initiating message.

    let bytes = hex::decode("35000004001b00080002f83910000102005240090300667265653567630066001000000000010002f839000010080102030015400140").unwrap();
    let mut data = AperCodecData::from_slice(&bytes);
    let ng_setup_2 = NgSetupRequest::decode(&mut data)?;
    println!("Yay {:?}", ng_setup_2);
    //assert!(ng_setup_2 == ng_setup);
    Ok(())
}
