use super::GnbcuT;
use crate::datastore::UeState;
use anyhow::{anyhow, Result};
use bitvec::prelude::*;
use f1ap::InitialUlRrcMessageTransfer;
use net::{AperSerde, Indication, Procedure};
use ngap::*;
use pdcp::PdcpPdu;
use rand::Rng;
use rrc::*;
use slog::{debug, Logger};

// Initial Access Procedure
pub async fn initial_access<G: GnbcuT>(
    gnbcu: &G,
    r: InitialUlRrcMessageTransfer,
    logger: &Logger,
) -> Result<()> {
    debug!(&logger, "Initial Access Procedure");

    let _rrc_setup_request = expect_rrc_setup_request(&r.rrc_container.0)?;

    let ue = UeState {
        amf_ue_ngap_id: None,
        gnb_du_ue_f1ap_id: r.gnb_du_ue_f1ap_id,
        key: rand::thread_rng().gen::<u32>(),
    };
    debug!(&logger, "Created UE {:#010x}", ue.key);

    let rrc_transaction = gnbcu.new_rrc_transaction(&ue).await;

    let rrc_setup = RrcSetup {
        rrc_transaction_identifier: RrcTransactionIdentifier(1),
        critical_extensions: CriticalExtensions21::RrcSetup(RrcSetupIEs {
            radio_bearer_config: RadioBearerConfig {
                srb_to_add_mod_list: None,
                srb_3_to_release: None,
                drb_to_add_mod_list: None,
                drb_to_release_list: None,
                security_config: None,
            },
            master_cell_group: vec![],
            late_non_critical_extension: None,
        }),
    };

    debug!(logger, "<< RrcSetup");
    let bytes = RrcSetupProcedure::encode_request(rrc_setup)?;
    let pdcp_pdu = PdcpPdu::encode(&bytes);
    let rrc_container = f1ap::RrcContainer(pdcp_pdu.into());
    gnbcu.send_rrc_to_ue(&ue, rrc_container, logger).await;
    let rrc_setup_complete = match rrc_transaction.recv().await?.message {
        UlDcchMessageType::C1(C1_6::RrcSetupComplete(x)) => Ok(x),
        _ => Err(anyhow!("Expected Rrc Setup complete")),
    }?;

    // This was an idea for a more elegant model.
    // let rrc_setup_complete = <UeRrcChannel as RequestProvider<RrcSetupProcedure>>::request(
    //     gnbcu.ue_rrc_channel(),
    //     rrc_setup,
    //     &logger,
    // )
    // .await
    // .map_err(|e| anyhow!(format!("Request error {:?}", e)))?;

    let rrc_setup_complete = match rrc_setup_complete.critical_extensions {
        CriticalExtensions22::RrcSetupComplete(x) => x,
    };

    // TODO: get establishment cause from the earlier Rrc Setup Request.
    let rrc_establishment_cause = RrcEstablishmentCause::MtAccess;

    // TODO: likewise get NrCgi from the F1AP UL Initial Transfer Request.  (Frunk-convert?)
    let nr_cgi = ngap::NrCgi {
        plmn_identity: ngap::PlmnIdentity(vec![0x02, 0xf8, 0x39]),
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
                    plmn_identity: ngap::PlmnIdentity(vec![0x02, 0xf8, 0x39]),
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
    InitialUeMessageProcedure::call_provider(gnbcu.ngap_stack(), m, logger).await;
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
