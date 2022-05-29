use super::{Gnbcu, UeContext};
use anyhow::Result;
use bitvec::prelude::*;
use f1ap::*;
use net::{AperSerde, Indication};
use ngap::*;
use rrc::*;
use slog::{debug, warn, Logger};

#[derive(Clone)]
pub struct RrcHandler(Gnbcu);

impl RrcHandler {
    pub fn new(gnbcu: Gnbcu) -> RrcHandler {
        RrcHandler(gnbcu)
    }

    pub async fn dispatch_ccch(&self, ue: UeContext, message: &[u8], logger: &Logger) {
        match match match match UlCcchMessage::from_bytes(message) {
            Err(e) => {
                warn!(logger, "Failed to decode RRC message: {:?}", e);
                return;
            }
            Ok(m) => m,
        }
        .message
        {
            UlCcchMessageType::C1(m) => m,
        } {
            C1_4::RrcSetupRequest(x) => self.rrc_setup_request(ue, x, logger),
            C1_4::RrcResumeRequest(_) => todo!(),
            C1_4::RrcReestablishmentRequest(_) => todo!(),
            C1_4::RrcSystemInfoRequest(_) => todo!(),
        }
        .await
        {
            Err(e) => warn!(logger, "Error processing Rrc message {:?}", e),
            _ => (),
        }
    }

    pub async fn dispatch_dcch(&self, ue: UeContext, message: &[u8], logger: &Logger) {
        match match match match UlDcchMessage::from_bytes(message) {
            Err(e) => {
                warn!(logger, "Failed to decode RRC message: {:?}", e);
                return;
            }
            Ok(m) => m,
        }
        .message
        {
            UlDcchMessageType::C1(m) => m,
            UlDcchMessageType::MessageClassExtension(_) => {
                warn!(
                    logger,
                    "UlDcchMessage ignored - message class extension not implemented"
                );
                return;
            }
        } {
            C1_6::RrcSetupComplete(x) => self.rrc_setup_complete(ue, x, logger),
            _ => todo!(),
        }
        .await
        {
            Err(e) => warn!(logger, "Error processing Rrc message {:?}", e),
            _ => (),
        }
    }

    pub async fn send_to_ue(&self, ue: UeContext, message: Vec<u8>, logger: &Logger) {
        let dl_message = DlRrcMessageTransfer {
            gnb_cu_ue_f1ap_id: ue.gnb_cu_ue_f1ap_id,
            gnb_du_ue_f1ap_id: ue.gnb_du_ue_f1ap_id,
            old_gnb_du_ue_f1ap_id: None,
            srb_id: SrbId(1),
            execute_duplication: None,
            rrc_container: f1ap::RrcContainer(message),
            rat_frequency_priority_information: None,
            rrc_delivery_status_request: None,
            ue_context_not_retrievable: None,
            redirected_rrc_message: None,
            plmn_assistance_info_for_net_shar: None,
            new_gnb_cu_ue_f1ap_id: None,
            additional_rrm_priority_index: None,
        };

        DlRrcMessageTransferProcedure::call_provider(&self.0.f1ap, dl_message, logger).await
    }

    async fn rrc_setup_request(
        &self,
        ue: UeContext,
        _req: RrcSetupRequest,
        logger: &Logger,
    ) -> Result<()> {
        let rrc_setup = DlCcchMessage {
            message: DlCcchMessageType::C1(C1_1::RrcSetup(RrcSetup {
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
            })),
        };

        // This has to be encapsulated in a PDCP PDU.
        let pdcp_pdu = pdcp::into_data_pdu(&rrc_setup.into_bytes()?);

        debug!(logger, "Send Rrc Setup");
        self.send_to_ue(ue, pdcp_pdu, logger).await;
        Ok(())
    }

    async fn rrc_setup_complete(
        &self,
        _ue: UeContext,
        req: RrcSetupComplete,
        logger: &Logger,
    ) -> Result<()> {
        // TODO: check transaction identifier matches that in UE context?
        let _transaction_id = req.rrc_transaction_identifier;
        let req = match req.critical_extensions {
            CriticalExtensions22::RrcSetupComplete(x) => x,
        };

        // TODO: get establishment cause from the earlier Rrc Setup Request.  Means
        // we need a single async fn / task that sends the Rrc Setup and waits for Rrc Setup Complete
        // with a timeout.  This means that the F1 layer needs to provide something
        // similar to impl<P: Procedure> RequestProvider<P> for Stack.
        let rrc_establishment_cause = RrcEstablishmentCause::MtAccess;

        // TODO: likewise get NrCgi from the F1AP UL Initial Transfer Request.  (Frunk-convert?)
        let nr_cgi = ngap::NrCgi {
            plmn_identity: ngap::PlmnIdentity(vec![0x02, 0xf8, 0x39]),
            nr_cell_identity: ngap::NrCellIdentity(bitvec![Msb0,u8;0;36]),
        };

        // Initial UE Message to the AMF containing the enclosed NAS message.
        let m = InitialUeMessage {
            ran_ue_ngap_id: RanUeNgapId(1),
            nas_pdu: NasPdu(req.dedicated_nas_message.0),
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

        InitialUeMessageProcedure::call_provider(&self.0.ngap, m, logger).await;

        Ok(())
    }
}
