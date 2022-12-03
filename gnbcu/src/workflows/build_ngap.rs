//! build_ngap - construction of NGAP messages

use crate::gnbcu_trait::Gnbcu;
use bitvec::prelude::*;
use ngap::{GlobalGnbId, GlobalRanNodeId, GnbId, PlmnIdentity};

pub fn build_global_ran_node_id<T: Gnbcu>(gnbcu: &T) -> GlobalRanNodeId {
    GlobalRanNodeId::GlobalGnbId(GlobalGnbId {
        plmn_identity: PlmnIdentity(gnbcu.config().plmn.clone()),
        gnb_id: GnbId::GnbId(bitvec![u8,Msb0; 1; 22]),
    })
}
