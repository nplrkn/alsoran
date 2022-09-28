//! initial_access - procedure in which UE makes first contact with the 5G core

use super::Gnbcu;
use crate::datastore::UeState;
use anyhow::{anyhow, Result};
use bitvec::prelude::*;
use f1ap::{InitialUlRrcMessageTransfer, SrbId};
use net::AperSerde;
use ngap::*;
use rrc::*;
use slog::{debug, Logger};

// Initial Access Procedure
// 1. >> Rrc RrcSetupRequest
// 2. << Rrc RrcSetup
// 3. >> Rrc RrcSetupComplete
// 4.    Ngap InitialUeMessage >>
pub async fn initial_access<G: Gnbcu>(
    gnbcu: &G,
    r: InitialUlRrcMessageTransfer,
    logger: &Logger,
) -> Result<()> {
    // TODO - "If the DU to CU RRC Container IE is not included in the INITIAL UL RRC MESSAGE TRANSFER,
    // the gNB-CU should reject the UE under the assumption that the gNB-DU is not able to serve such UE."

    // TODO - "If the RRC-Container-RRCSetupComplete IE is included in the INITIAL UL RRC MESSAGE TRANSFER,
    // the gNB-CU shall take it into account as specified in TS 38.401 [4]."

    let _rrc_setup_request = expect_rrc_setup_request(&r.rrc_container.0)?;
    debug!(logger, ">> Rrc RrcSetupRequest");

    let ue = UeState::new(r.gnb_du_ue_f1ap_id);
    debug!(&logger, "Created UE {:#010x}", ue.key);

    let rrc_transaction = gnbcu.new_rrc_transaction(&ue).await;
    let rrc_setup = super::build_rrc::build_rrc_setup(1)?;

    debug!(logger, "<< RrcSetup");
    gnbcu.send_rrc_to_ue(&ue, SrbId(0), rrc_setup, logger).await;
    let rrc_setup_complete = match rrc_transaction.recv().await?.message {
        UlDcchMessageType::C1(C1_6::RrcSetupComplete(x)) => Ok(x),
        _ => Err(anyhow!("Expected Rrc Setup complete")),
    }?;

    let rrc_setup_complete = match rrc_setup_complete.critical_extensions {
        CriticalExtensions22::RrcSetupComplete(x) => x,
    };

    // TODO: get establishment cause from the earlier Rrc Setup Request.
    let rrc_establishment_cause = RrcEstablishmentCause::MtAccess;

    // TODO: likewise get NrCgi from the F1AP UL Initial Transfer Request.  (Frunk transmogrify ideally.)
    let nr_cgi = ngap::NrCgi {
        plmn_identity: ngap::PlmnIdentity(gnbcu.config().plmn.clone()),
        nr_cell_identity: ngap::NrCellIdentity(bitvec![u8,Msb0;0;36]),
    };

    // Initial UE Message to the AMF containing the enclosed NAS message.
    let m = InitialUeMessage {
        ran_ue_ngap_id: RanUeNgapId(ue.key),
        nas_pdu: NasPdu(rrc_setup_complete.dedicated_nas_message.0),
        user_location_information: UserLocationInformation::UserLocationInformationNr(
            UserLocationInformationNr {
                nr_cgi,
                tai: Tai {
                    plmn_identity: ngap::PlmnIdentity(gnbcu.config().plmn.clone()),
                    tac: Tac(vec![0, 0, 1]),
                },
                time_stamp: None,
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
    };

    debug!(logger, "Store UE {:#010x}", ue.key);
    gnbcu
        .store(ue.key, ue, gnbcu.config().initial_ue_ttl_secs)
        .await?;

    debug!(logger, "InitialUeMessage(Nas) >>");
    gnbcu
        .ngap_indication::<InitialUeMessageProcedure>(m, logger)
        .await;
    Ok(())
}

fn expect_rrc_setup_request(message: &[u8]) -> Result<RrcSetupRequest> {
    match UlCcchMessage::from_bytes(message)? {
        UlCcchMessage {
            message: UlCcchMessageType::C1(C1_4::RrcSetupRequest(x)),
        } => Ok(x),
        m => Err(anyhow!(format!("Not yet implemented Rrc message {:?}", m))),
    }
}
