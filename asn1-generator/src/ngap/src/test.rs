use super::common::*;
use super::ies::*;
use super::pdu::*;
use asn1::aper::{APerElement, EncodeError, UNCONSTRAINED};
use asn1::BitString;

#[test]
fn test_ng_setup_encode() -> Result<(), EncodeError> {
    let plmn_identity = PlmnIdentity(vec![0x02, 0xf8, 0x39]);
    let ng_setup = NgSetupRequest {
        global_ran_node_id: GlobalRanNodeId::GlobalGnbId(GlobalGnbId {
            plmn_identity: plmn_identity.clone(),
            gnb_id: GnbId::GnbId(BitString::with_len(1)),
        }),
        ran_node_name: Some(RanNodeName(PrintableString("free5GC".to_string()))),
        supported_ta_list: SupportedTaList(vec![SupportedTaItem {
            tac: Tac(vec![0x1]),
            broadcast_plmn_list: BroadcastPlmnList(vec![BroadcastPlmnItem {
                plmn_identity: plmn_identity,
                tai_slice_support_list: SliceSupportList(vec![SliceSupportItem {
                    s_nssai: SNssai {
                        sst: Sst(vec![0x1]),
                        sd: None,
                    },
                }]),
            }]),
        }]),
        default_paging_drx: PagingDrx::V128,
        ue_retention_information: None,
        nb_iot_default_paging_drx: None,
        extended_ran_node_name: None,
    };
    let encoding = ng_setup.to_aper(UNCONSTRAINED)?;
    println!("{:?}", encoding);
    Ok(())
}
