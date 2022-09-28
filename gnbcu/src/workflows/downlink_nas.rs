//! downlink_nas - transfer of a Nas message from AMF to UE

use crate::datastore::UeState;

use super::Gnbcu;
use anyhow::Result;
use f1ap::SrbId;
use ngap::{AmfUeNgapId, DownlinkNasTransport};
use rrc::*;
use slog::{debug, Logger};

// Downlink Nas Procedure
// 1.    Ngap DownlinkNasTransport(Nas) <<
// 2. << Rrc DlInformationTransfer(Nas)
pub async fn downlink_nas<G: Gnbcu>(
    gnbcu: &G,
    i: DownlinkNasTransport,
    logger: &Logger,
) -> Result<()> {
    debug!(&logger, "DownlinkNasTransport(Nas) <<");
    let mut ue = gnbcu.retrieve(&i.ran_ue_ngap_id.0).await?;
    maybe_learn_amf_ngap_id(gnbcu, &mut ue, i.amf_ue_ngap_id, logger).await;
    send_nas_to_ue(gnbcu, &ue, DedicatedNasMessage(i.nas_pdu.0), logger).await
}

// If we don't already know the AMF's ID for this UE, save it off now.
async fn maybe_learn_amf_ngap_id<G: Gnbcu>(
    gnbcu: &G,
    ue: &mut UeState,
    amf_ue_ngap_id: AmfUeNgapId,
    logger: &Logger,
) {
    match ue.amf_ue_ngap_id {
        Some(ref x) if x.0 == amf_ue_ngap_id.0 => (),
        _ => {
            debug!(logger, "Learned AMF NGAP ID {:#x}", amf_ue_ngap_id.0);
            ue.amf_ue_ngap_id = Some(amf_ue_ngap_id);
            if let Err(e) = gnbcu
                .store(ue.key, ue.clone(), gnbcu.config().initial_ue_ttl_secs)
                .await
            {
                // We soldier on here as we might as well send the message onwards to the UE anyway.
                debug!(logger, "Failed to store UE state - error {:?}", e);
            }
        }
    }
}

pub async fn send_nas_to_ue<G: Gnbcu>(
    gnbcu: &G,
    ue: &UeState,
    nas: DedicatedNasMessage,
    logger: &Logger,
) -> Result<()> {
    let rrc_container = super::build_rrc::build_rrc_dl_information_transfer(2, Some(nas))?;

    debug!(&logger, "<< DlInformationTransfer(Nas)");
    gnbcu
        .send_rrc_to_ue(&ue, SrbId(1), rrc_container, logger)
        .await;
    Ok(())
}
