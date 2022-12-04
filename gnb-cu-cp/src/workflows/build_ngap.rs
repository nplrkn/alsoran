//! build_ngap - construction of NGAP messages

use crate::gnb_cu_cp::GnbCuCp;
use bitvec::prelude::*;
use ngap::{GlobalGnbId, GlobalRanNodeId, GnbId, PlmnIdentity};

pub fn build_global_ran_node_id<T: GnbCuCp>(gnb_cu_cp: &T) -> GlobalRanNodeId {
    GlobalRanNodeId::GlobalGnbId(GlobalGnbId {
        plmn_identity: PlmnIdentity(gnb_cu_cp.config().plmn.clone()),
        gnb_id: GnbId::GnbId(bitvec![u8,Msb0; 1; 22]),
    })
}
