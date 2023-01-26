//! uplink_nas - transfer of a Nas message from UE to AMF

use super::{GnbCuCp, Workflow};
use crate::datastore::UeState;
use bitvec::prelude::*;
use ngap::{
    NasPdu, RanUeNgapId, Tac, Tai, UplinkNasTransport, UplinkNasTransportProcedure,
    UserLocationInformation, UserLocationInformationNr,
};
use rrc::*;
use slog::debug;

impl<'a, G: GnbCuCp> Workflow<'a, G> {
    // Uplink Nas Procedure
    // 1. >> Rrc UlInformationTransfer(Nas)
    // 2.    Ngap UplinkNasTransport(Nas) >>
    pub async fn uplink_nas(&self, ue: UeState, i: UlInformationTransfer) {
        let nas_pdu = match i.critical_extensions {
            CriticalExtensions37::UlInformationTransfer(UlInformationTransferIEs {
                dedicated_nas_message: Some(x),
                ..
            }) => NasPdu(x.0),
            _ => {
                debug!(self.logger, "No Nas Message present - nothing to do");
                return;
            }
        };

        self.log_message(">> UlInformationTransfer(Nas)");

        let amf_ue_ngap_id = match ue.amf_ue_ngap_id {
            Some(x) => x,
            None => {
                debug!(
                    self.logger,
                    "AMF NGAP Id not present - can't send Uplink Nas Transport"
                );
                return;
            }
        };

        // Todo - should be from Ue context
        let nr_cgi = ngap::NrCgi {
            plmn_identity: ngap::PlmnIdentity(self.config().plmn.clone()),
            nr_cell_identity: ngap::NrCellIdentity(bitvec![u8,Msb0;0;36]),
        };

        let m = UplinkNasTransport {
            amf_ue_ngap_id,
            ran_ue_ngap_id: RanUeNgapId(ue.key),
            nas_pdu,
            user_location_information: UserLocationInformation::UserLocationInformationNr(
                UserLocationInformationNr {
                    nr_cgi,
                    tai: Tai {
                        plmn_identity: ngap::PlmnIdentity(self.config().plmn.clone()),
                        tac: Tac(vec![0, 0, 1]),
                    },
                    time_stamp: None,
                    ps_cell_information: None,
                    nid: None,
                },
            ),
            w_agf_identity_information: None,
            tngf_identity_information: None,
            twif_identity_information: None,
        };

        self.log_message("UplinkNasTransport(Nas) >>");
        self.ngap_indication::<UplinkNasTransportProcedure>(m, self.logger)
            .await
    }
}
