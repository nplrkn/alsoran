use crate::GnbcuOps;
use bitvec::prelude::*;
use net::{RequestProvider, Stack};
use ngap::*;
use slog::{debug, info, warn, Logger};

pub async fn ng_setup<G: GnbcuOps>(gnbcu: &G, logger: &Logger) {
    // This uses the default expected values of free5GC.
    let ng_setup_request = NgSetupRequest {
        global_ran_node_id: GlobalRanNodeId::GlobalGnbId(GlobalGnbId {
            plmn_identity: PlmnIdentity(vec![0x2, 0xf8, 0x39]),
            gnb_id: GnbId::GnbId(bitvec![u8,Msb0; 1; 22]),
        }),
        ran_node_name: None,
        supported_ta_list: SupportedTaList(vec![SupportedTaItem {
            tac: Tac(vec![0x0, 0x0, 0x1]),
            broadcast_plmn_list: BroadcastPlmnList(vec![BroadcastPlmnItem {
                plmn_identity: PlmnIdentity(vec![0x2, 0xf8, 0x39]),
                tai_slice_support_list: SliceSupportList(vec![SliceSupportItem {
                    s_nssai: ngap::SNssai {
                        sst: Sst(vec![0x01]),
                        sd: Some(Sd(vec![0x1, 0x2, 0x3])),
                    },
                }]),
            }]),
        }]),
        default_paging_drx: PagingDrx::V128,
        ue_retention_information: None,
        nb_iot_default_paging_drx: None,
        extended_ran_node_name: None,
    };
    let ng_setup_provider = gnbcu.ngap_stack();
    debug!(logger, "NgSetupRequest >>");
    match <Stack as RequestProvider<NgSetupProcedure>>::request(
        ng_setup_provider,
        ng_setup_request,
        logger,
    )
    .await
    {
        Ok(response) => {
            debug!(logger, "NgSetupResponse <<");
            info!(
                logger,
                "NGAP interface initialized with {:?}", response.amf_name
            );
        }

        Err(e) => warn!(logger, "NG Setup failed - {:?}", e),
    }
}
