use super::GnbcuT;
use crate::datastore::UeState;
use bitvec::prelude::*;
use ngap::{
    NasPdu, RanUeNgapId, Tac, Tai, UplinkNasTransport, UplinkNasTransportProcedure,
    UserLocationInformation, UserLocationInformationNr,
};
use rrc::*;
use slog::{debug, Logger};

// Uplink Nas Procedure
pub async fn uplink_nas<G: GnbcuT>(
    gnbcu: &G,
    ue: UeState,
    i: UlInformationTransfer,
    logger: &Logger,
) {
    let nas_pdu = match i.critical_extensions {
        CriticalExtensions37::UlInformationTransfer(UlInformationTransferIEs {
            dedicated_nas_message: Some(x),
            ..
        }) => NasPdu(x.0),
        _ => {
            debug!(&logger, "No Nas Message present - nothing to do");
            return;
        }
    };

    debug!(logger, ">> UlInformationTransfer(Nas)");

    let amf_ue_ngap_id = match ue.amf_ue_ngap_id {
        Some(x) => x,
        None => {
            debug!(
                &logger,
                "AMF NGAP Id not present - can't send Uplink Nas Transport"
            );
            return;
        }
    };

    // Todo - should be from Ue context
    let nr_cgi = ngap::NrCgi {
        plmn_identity: ngap::PlmnIdentity(vec![0x02, 0xf8, 0x39]),
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
                    plmn_identity: ngap::PlmnIdentity(vec![0x02, 0xf8, 0x39]),
                    tac: Tac(vec![0, 0, 1]),
                },
                time_stamp: None,
            },
        ),
        w_agf_identity_information: None,
        tngf_identity_information: None,
        twif_identity_information: None,
    };

    debug!(logger, "UplinkNasTransport(Nas) >>");
    gnbcu
        .ngap_indication::<UplinkNasTransportProcedure>(m, logger)
        .await
}
