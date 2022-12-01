//! ng_setup - the initial handshake that establishes an instance of the NG reference point between GNB and AMF

use super::{Gnbcu, Workflow};
use anyhow::Result;
use ngap::*;
use slog::info;

impl<'a, G: Gnbcu> Workflow<'a, G> {
    // Ng Setup Procedure
    // 1.    Ngap NgSetupRequest >>
    // 2.    Ngap NgSetupResponse <<
    pub async fn ng_setup(&self) -> Result<AmfName> {
        // This uses the default expected values of free5GC.
        let ng_setup_request = NgSetupRequest {
            global_ran_node_id: super::build_ngap::build_global_ran_node_id(self.gnbcu),
            ran_node_name: self.config().name.clone().map(RanNodeName),
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
        let response = self
            .ngap_request::<NgSetupProcedure>(ng_setup_request, self.logger)
            .await?;
        self.log_message("NgSetupResponse <<");
        info!(
            self.logger,
            "NGAP interface initialized with {:?}", response.amf_name
        );

        // Associate this TNLA with the NGAP interface instance.
        //self.associate_connection();

        Ok(response.amf_name)
    }
}
