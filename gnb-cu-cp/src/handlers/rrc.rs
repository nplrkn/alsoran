//! rrc - RRC entry points into the GNB-CU

use crate::{workflows::Workflow, GnbCuCp, UeState};
use anyhow::Result;
use net::SerDes;
use rrc::*;
use slog::{debug, warn, Logger};

#[derive(Clone)]
pub struct RrcHandler<G: GnbCuCp> {
    gnb_cu_cp: G,
}

impl<G: GnbCuCp> RrcHandler<G> {
    pub fn new(gnb_cu_cp: G) -> RrcHandler<G> {
        RrcHandler { gnb_cu_cp }
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
        if let Some(sender) = self.gnb_cu_cp.match_rrc_transaction(ue_id).await {
            let _ = sender.send(message).await;
            return;
        }

        // This is a request.  Retrieve the UE.
        let ue = match self.gnb_cu_cp.retrieve(&ue_id).await {
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
        Workflow::new(&self.gnb_cu_cp, logger)
            .uplink_nas(ue, req)
            .await;
        Ok(())
    }
}
