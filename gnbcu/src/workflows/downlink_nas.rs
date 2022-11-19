//! downlink_nas - transfer of a Nas message from AMF to UE

use crate::datastore::UeState;

use super::{Gnbcu, Workflow};
use anyhow::Result;
use f1ap::SrbId;
use ngap::{AmfUeNgapId, DownlinkNasTransport};
use rrc::*;
use slog::debug;

impl<'a, G: Gnbcu> Workflow<'a, G> {
    // Downlink Nas Procedure
    // 1.    Ngap DownlinkNasTransport(Nas) <<
    // 2. << Rrc DlInformationTransfer(Nas)
    pub async fn downlink_nas(&self, i: DownlinkNasTransport) -> Result<()> {
        self.log_message("DownlinkNasTransport(Nas) <<");

        let mut ue = self.retrieve(&i.ran_ue_ngap_id.0).await?;

        self.maybe_learn_amf_ngap_id(&mut ue, i.amf_ue_ngap_id)
            .await;

        self.send_nas_to_ue(&ue, DedicatedNasMessage(i.nas_pdu.0))
            .await
    }

    // If we don't already know the AMF's ID for this UE, save it off now.
    async fn maybe_learn_amf_ngap_id(&self, ue: &mut UeState, amf_ue_ngap_id: AmfUeNgapId) {
        match ue.amf_ue_ngap_id {
            Some(ref x) if x.0 == amf_ue_ngap_id.0 => (),
            _ => {
                debug!(self.logger, "Learned AMF NGAP ID {:#x}", amf_ue_ngap_id.0);
                ue.amf_ue_ngap_id = Some(amf_ue_ngap_id);
                if let Err(e) = self
                    .gnbcu
                    .store(ue.key, ue.clone(), self.config().initial_ue_ttl_secs)
                    .await
                {
                    // We soldier on here as we might as well send the message onwards to the UE anyway.
                    debug!(self.logger, "Failed to store UE state - error {:?}", e);
                }
            }
        }
    }

    pub async fn send_nas_to_ue(&self, ue: &UeState, nas: DedicatedNasMessage) -> Result<()> {
        let rrc_container = super::build_rrc::build_rrc_dl_information_transfer(2, Some(nas))?;

        self.log_message("<< DlInformationTransfer(Nas)");
        self.send_rrc_to_ue(ue, SrbId(1), rrc_container, self.logger)
            .await;
        Ok(())
    }
}
