//! initial_access - procedure in which UE makes first contact with the 5G core

use super::{GnbCuCp, Workflow};
use crate::datastore::UeState;
use anyhow::{anyhow, Result};
use bitvec::prelude::*;
use f1ap::{InitialUlRrcMessageTransfer, SrbId};
use net::AperSerde;
use ngap::*;
use rrc::*;
use slog::debug;

impl<'a, G: GnbCuCp> Workflow<'a, G> {
    // Initial Access Procedure
    // 1. >> Rrc RrcSetupRequest
    // 2. << Rrc RrcSetup
    // 3. >> Rrc RrcSetupComplete
    // 4.    Ngap InitialUeMessage >>
    pub async fn initial_access(&self, r: InitialUlRrcMessageTransfer) -> Result<()> {
        self.log_message(">> InitialUlRrcMessageTransfer");

        let _rrc_setup_request = expect_rrc_setup_request(&r.rrc_container.0)?;
        self.log_message(">> Rrc RrcSetupRequest");

        let ue = UeState::new(r.gnb_du_ue_f1ap_id);
        debug!(self.logger, "Created UE {:#010x}", ue.key);

        let rrc_setup_complete = self.perform_rrc_setup_procedure(&ue).await?;

        let initial_ue_message = self.build_initial_ue_message(&ue, rrc_setup_complete);

        debug!(self.logger, "Store UE {:#010x}", ue.key);
        self.store(ue.key, ue, self.config().initial_ue_ttl_secs)
            .await?;

        self.log_message("InitialUeMessage(Nas) >>");
        self.ngap_indication::<InitialUeMessageProcedure>(initial_ue_message, self.logger)
            .await;
        Ok(())
    }

    async fn perform_rrc_setup_procedure(&self, ue: &UeState) -> Result<RrcSetupCompleteIEs> {
        let rrc_transaction = self.gnb_cu_cp.new_rrc_transaction(ue).await;
        let rrc_setup = super::build_rrc::build_rrc_setup(1)?;

        self.log_message("<< RrcSetup");
        self.send_rrc_to_ue(ue, SrbId(0), rrc_setup, self.logger)
            .await;

        rrc_transaction.recv().await.and_then(|x| match x.message {
            UlDcchMessageType::C1(C1_6::RrcSetupComplete(RrcSetupComplete {
                critical_extensions: CriticalExtensions22::RrcSetupComplete(x),
                ..
            })) => Ok(x),
            _ => Err(anyhow!("Expected Rrc Setup complete")),
        })
    }

    fn build_initial_ue_message(
        &self,
        ue: &UeState,
        rrc_setup_complete: RrcSetupCompleteIEs,
    ) -> InitialUeMessage {
        // TODO: get establishment cause from the earlier Rrc Setup Request.
        let rrc_establishment_cause = RrcEstablishmentCause::MtAccess;

        // TODO: likewise get NrCgi from the F1AP UL Initial Transfer Request.  (Frunk transmogrify ideally.)
        let nr_cgi = ngap::NrCgi {
            plmn_identity: ngap::PlmnIdentity(self.gnb_cu_cp.config().plmn.clone()),
            nr_cell_identity: ngap::NrCellIdentity(bitvec![u8,Msb0;0;36]),
        };

        // Initial UE Message to the AMF containing the enclosed NAS message.
        InitialUeMessage {
            ran_ue_ngap_id: RanUeNgapId(ue.key),
            nas_pdu: NasPdu(rrc_setup_complete.dedicated_nas_message.0),
            user_location_information: UserLocationInformation::UserLocationInformationNr(
                UserLocationInformationNr {
                    nr_cgi,
                    tai: Tai {
                        plmn_identity: ngap::PlmnIdentity(self.gnb_cu_cp.config().plmn.clone()),
                        tac: Tac(vec![0, 0, 1]),
                    },
                    time_stamp: None,
                    ps_cell_information: None,
                    nid: None,
                },
            ),
            rrc_establishment_cause,
            five_g_s_tmsi: None,
            amf_set_id: None,
            ue_context_request: Some(UeContextRequest::Requested),
            allowed_nssai: None,
            source_to_target_amf_information_reroute: None,
            selected_plmn_identity: None,
            iab_node_indication: None,
            c_emode_b_support_indicator: None,
            ltem_indication: None,
            edt_session: None,
            authenticated_indication: None,
            npn_access_information: None,
        }
    }
}

fn expect_rrc_setup_request(message: &[u8]) -> Result<RrcSetupRequest> {
    match UlCcchMessage::from_bytes(message)? {
        UlCcchMessage {
            message: UlCcchMessageType::C1(C1_4::RrcSetupRequest(x)),
        } => Ok(x),
        m => Err(anyhow!(format!("Not yet implemented Rrc message {:?}", m))),
    }
}
