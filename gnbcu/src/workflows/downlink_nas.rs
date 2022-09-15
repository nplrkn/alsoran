//! downlink_nas - transfer of a Nas message from AMF to UE

use crate::datastore::UeState;

use super::Gnbcu;
use net::AperSerde;
use ngap::DownlinkNasTransport;
use pdcp::PdcpPdu;
use rrc::*;
use slog::{debug, warn, Logger};

// Downlink Nas Procedure
// 1.    Ngap DownlinkNasTransport(Nas) <<
// 2. << Rrc DlInformationTransfer(Nas)
pub async fn downlink_nas<G: Gnbcu>(gnbcu: &G, i: DownlinkNasTransport, logger: &Logger) {
    debug!(&logger, "DownlinkNasTransport(Nas) <<");

    let mut ue = match gnbcu.retrieve(&i.ran_ue_ngap_id.0).await {
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

    match ue.amf_ue_ngap_id {
        Some(ref x) if x.0 == i.amf_ue_ngap_id.0 => (),
        _ => {
            debug!(logger, "Learned AMF NGAP ID {:#x}", i.amf_ue_ngap_id.0);
            ue.amf_ue_ngap_id = Some(i.amf_ue_ngap_id);
            if let Err(e) = gnbcu
                .store(ue.key, ue.clone(), gnbcu.config().initial_ue_ttl_secs)
                .await
            {
                debug!(logger, "Failed to store UE state - error {:?}", e);
            }
        }
    }
    send_nas_to_ue(gnbcu, &ue, DedicatedNasMessage(i.nas_pdu.0), logger).await
}

pub async fn send_nas_to_ue<G: Gnbcu>(
    gnbcu: &G,
    ue: &UeState,
    nas: DedicatedNasMessage,
    logger: &Logger,
) {
    let rrc = match (DlDcchMessage {
        message: DlDcchMessageType::C1(C1_2::DlInformationTransfer(DlInformationTransfer {
            rrc_transaction_identifier: RrcTransactionIdentifier(2),
            critical_extensions: CriticalExtensions4::DlInformationTransfer(
                DlInformationTransferIEs {
                    dedicated_nas_message: Some(nas),
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
    gnbcu.send_rrc_to_ue(&ue, rrc_container, logger).await;
}
