//! rrc - RRC entry points into the GNB-CU

use crate::{Gnbcu, UeState};
use anyhow::Result;
use net::AperSerde;
use rrc::*;
use slog::{debug, warn, Logger};

#[derive(Clone)]
pub struct RrcHandler<G: Gnbcu> {
    gnbcu: G,
}

impl<G: Gnbcu> RrcHandler<G> {
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
        crate::workflows::uplink_nas(&self.gnbcu, ue, req, logger).await;
        Ok(())
    }
}
