use super::ies::*;
use super::pdu::*;
use asn1::aper::encode_int;
use asn1::aper::{APerElement, EncodeError, Encoding, UNCONSTRAINED};
use asn1::{BitString, PrintableString};

#[test]
fn test_ng_setup_encode() -> Result<(), EncodeError> {
    let plmn_identity = PlmnIdentity(vec![0x02, 0xf8, 0x39]);
    let ng_setup = NgSetupRequest {
        global_ran_node_id: GlobalRanNodeId::GlobalGnbId(GlobalGnbId {
            plmn_identity: plmn_identity.clone(),
            gnb_id: GnbId::GnbId(BitString::with_bytes_and_len(&vec![0x00, 0x01, 0x02], 3)),
        }),
        ran_node_name: Some(RanNodeName(
            PrintableString::from_string("free5GC".to_string()).unwrap(),
        )),
        supported_ta_list: SupportedTaList(vec![SupportedTaItem {
            tac: Tac(vec![0x00, 0x00, 0x01]),
            broadcast_plmn_list: BroadcastPlmnList(vec![BroadcastPlmnItem {
                plmn_identity: plmn_identity,
                tai_slice_support_list: SliceSupportList(vec![SliceSupportItem {
                    s_nssai: SNssai {
                        sst: Sst(vec![0x01]),
                        sd: Some(Sd(vec![0x01, 0x02, 0x03])),
                    },
                }]),
            }]),
        }]),
        default_paging_drx: PagingDrx::V128,
        ue_retention_information: None,
        nb_iot_default_paging_drx: None,
        extended_ran_node_name: None,
    };
    let mut enc = Encoding::new();
    // initiating message - extensible choice 0-2
    false.to_aper(&mut enc, UNCONSTRAINED)?;
    enc.append(&encode_int(0, Some(0), Some(2))?)?;
    // procedure code 21
    enc.align();
    enc.append(&encode_int(21, Some(0), Some(255))?)?;
    enc.append(&encode_int(0, Some(0), Some(2))?)?;

    enc.append_open(&ng_setup)?;

    println!("{:02x?}", enc.bytes());
    Ok(())
}
