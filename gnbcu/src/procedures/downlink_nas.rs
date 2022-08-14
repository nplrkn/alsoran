use crate::GnbcuOps;
use net::AperSerde;
use ngap::DownlinkNasTransport;
use pdcp::PdcpPdu;
use rrc::*;
use slog::{debug, warn, Logger};

// Downlink Nas Procedure
pub async fn downlink_nas<G: GnbcuOps>(gnbcu: &G, i: DownlinkNasTransport, logger: &Logger) {
    debug!(&logger, "DownlinkNasTransport(Nas) <<");

    let ue_state = match gnbcu.retrieve(&i.ran_ue_ngap_id.0).await {
        Ok(x) => x,
        _ => {
            debug!(
                &logger,
                "Failed to get UE {:#010x} - can't carry out downlink Nas transfer",
                i.ran_ue_ngap_id.0
            );
            return;
        }
    };

    let rrc = match (DlDcchMessage {
        message: DlDcchMessageType::C1(C1_2::DlInformationTransfer(DlInformationTransfer {
            rrc_transaction_identifier: RrcTransactionIdentifier(2),
            critical_extensions: CriticalExtensions4::DlInformationTransfer(
                DlInformationTransferIEs {
                    dedicated_nas_message: Some(DedicatedNasMessage(i.nas_pdu.0)),
                    late_non_critical_extension: None,
                    non_critical_extension: None,
                },
            ),
        })),
    }
    .into_bytes())
    {
        Ok(x) => x,
        Err(e) => {
            warn!(
                logger,
                "Failed to encode Rrc DlInformationTransfer - {:?}", e
            );
            return;
        }
    };
    let rrc_container = f1ap::RrcContainer(PdcpPdu::encode(&rrc).into());
    debug!(&logger, "<< DlInformationTransfer(Nas)");
    gnbcu.send_rrc_to_ue(&ue_state, rrc_container, logger).await;
}
