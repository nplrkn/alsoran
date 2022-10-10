//! ng_setup - the initial handshake that establishes an instance of the NG reference point between GNB and AMF

use super::{Gnbcu, Workflow};
use bitvec::prelude::*;
use ngap::*;
use slog::{info, warn};

impl<'a, G: Gnbcu> Workflow<'a, G> {
    // Ng Setup Procedure
    // 1.    Ngap NgSetupRequest >>
    // 2.    Ngap NgSetupResponse <<
    pub async fn ng_setup(&self) {
        // This uses the default expected values of free5GC.
        let ng_setup_request = NgSetupRequest {
            global_ran_node_id: GlobalRanNodeId::GlobalGnbId(GlobalGnbId {
                plmn_identity: PlmnIdentity(self.config().plmn.clone()),
                gnb_id: GnbId::GnbId(bitvec![u8,Msb0; 1; 22]),
            }),
            ran_node_name: self.config().clone().name.map(|x| RanNodeName(x)),
            supported_ta_list: SupportedTaList(vec![SupportedTaItem {
                tac: Tac(vec![0x0, 0x0, 0x1]),
                broadcast_plmn_list: BroadcastPlmnList(vec![BroadcastPlmnItem {
                    plmn_identity: PlmnIdentity(self.config().plmn.clone()),
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
        self.log_message("NgSetupRequest >>");
        match self
            .ngap_request::<NgSetupProcedure>(ng_setup_request, self.logger)
            .await
        {
            Ok(response) => {
                self.log_message("NgSetupResponse <<");
                info!(
                    self.logger,
                    "NGAP interface initialized with {:?}", response.amf_name
                );
            }

            Err(e) => warn!(self.logger, "NG Setup failed - {:?}", e),
        };
    }
}
