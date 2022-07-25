use crate::GnbcuOps;
use crate::UeState;
use anyhow::{anyhow, Result};
use bitvec::prelude::*;
use net::{AperSerde, Indication};
use ngap::*;

use rrc::*;
use slog::{debug, warn, Logger};

#[derive(Clone)]
pub struct RrcHandler<G: GnbcuOps> {
    gnbcu: G,
}

impl<G: GnbcuOps> RrcHandler<G> {
    pub fn new(gnbcu: G) -> RrcHandler<G> {
        RrcHandler { gnbcu }
    }

    pub async fn dispatch_dcch(&self, ue_id: u32, message: &[u8], logger: &Logger) {
        let message = match UlDcchMessage::from_bytes(message) {
            Err(e) => {
                warn!(logger, "Failed to decode RRC message: {:?}", e);
                return;
            }
            Ok(m) => m,
        };

        // Look for a matching transaction.
        if let Some(sender) = self.gnbcu.match_rrc_transaction(ue_id).await {
            let _ = sender.send(message).await;
            return;
        }

        // This is a request.  Retrieve the UE.
        let ue = match self.gnbcu.retrieve(&ue_id).await {
            Ok(x) => x,
            _ => {
                debug!(
                    &logger,
                    "Failed to get UE {:#010x} - can't carry out UL message transfer", ue_id
                );
                return;
            }
        };

        match message.message {
            UlDcchMessageType::C1(C1_6::UlInformationTransfer(x)) => {
                if let Err(e) = self.ul_information_transfer(ue, x, logger).await {
                    warn!(logger, "Error processing Ul Information Transfer - {:?}", e)
                }
            }
            _ => warn!(logger, "Unsupported UlDcchMessage {:?}", message.message),
        }
    }

    async fn ul_information_transfer(
        &self,
        ue: UeState,
        req: UlInformationTransfer,
        logger: &Logger,
    ) -> Result<()> {
        let nas_pdu = match req.critical_extensions {
            CriticalExtensions37::UlInformationTransfer(UlInformationTransferIEs {
                dedicated_nas_message: Some(x),
                ..
            }) => NasPdu(x.0),
            _ => {
                debug!(&logger, "No Nas Message present - nothing to do");
                return Ok(());
            }
        };

        debug!(logger, ">> UlInformationTransfer(Nas)");

        let amf_ue_ngap_id = ue.amf_ue_ngap_id.ok_or(anyhow!("AMF NGAP Id unknown"))?;

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
        UplinkNasTransportProcedure::call_provider(self.gnbcu.ngap_stack(), m, logger).await;
        Ok(())
    }
}
