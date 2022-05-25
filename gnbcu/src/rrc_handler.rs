use super::{Gnbcu, UeContext};
use anyhow::Result;
use f1ap::*;
use net::{AperSerde, Procedure};
use rrc::*;
use slog::{trace, warn, Logger};

#[derive(Clone)]
pub struct RrcHandler(Gnbcu);

impl RrcHandler {
    pub fn new(gnbcu: Gnbcu) -> RrcHandler {
        RrcHandler(gnbcu)
    }

    pub async fn dispatch(&self, ue: UeContext, message: &[u8], logger: &Logger) {
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

    pub async fn send_to_ue(&self, ue: UeContext, message: Vec<u8>, logger: &Logger) -> Result<()> {
        let dl_message = DlRrcMessageTransfer {
            gnb_cu_ue_f1ap_id: ue.gnb_cu_ue_f1ap_id,
            gnb_du_ue_f1ap_id: ue.gnb_du_ue_f1ap_id,
            old_gnb_du_ue_f1ap_id: None,
            srb_id: SrbId(1),
            execute_duplication: None,
            rrc_container: RrcContainer(message),
            rat_frequency_priority_information: None,
            rrc_delivery_status_request: None,
            ue_context_not_retrievable: None,
            redirected_rrc_message: None,
            plmn_assistance_info_for_net_shar: None,
            new_gnb_cu_ue_f1ap_id: None,
            additional_rrm_priority_index: None,
        };

        let _ =
            DlRrcMessageTransferProcedure::call_provider(&self.0.f1ap, dl_message, logger).await;
        Ok(())
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
        trace!(logger, "Send Rrc Setup");
        self.send_to_ue(ue, rrc_setup.into_bytes()?, logger).await
    }
}
