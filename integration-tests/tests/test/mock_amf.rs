use anyhow::{anyhow, Result};
use async_channel::{Receiver, Sender};
use async_std::task::JoinHandle;
use async_trait::async_trait;
use bitvec::prelude::BitVec;
use net::TransportProvider;
use net::{
    Asn1PerCodec, SctpTransportProvider, ServerTransportProvider, TnlaEvent, TnlaEventHandler,
};
use ngap::NgapPdu;
use ngap::*;
use slog::info;
use slog::{o, trace, Logger};
use std::fmt::Debug;
use stop_token::StopSource;

const NGAP_SCTP_PPID: u32 = 60;

// TODO why pub?
pub struct MockAmf {
    pub stop_source: StopSource,
    pub receiver: Receiver<Option<NgapPdu>>,
    pub sender: SctpTransportProvider<Asn1PerCodec<NgapPdu>, NgapPdu>,
    pub task: JoinHandle<()>,
    logger: Logger,
}

#[derive(Debug, Clone)]
struct Handler(pub Sender<Option<NgapPdu>>);

impl MockAmf {
    pub async fn new(amf_address: &str, logger: &Logger) -> MockAmf {
        let (internal_sender, receiver) = async_channel::unbounded();
        let logger = logger.new(o!("amf" => 1));
        let stop_source = StopSource::new();
        let server = SctpTransportProvider::new(NGAP_SCTP_PPID, Asn1PerCodec::new());
        let task = server
            .clone()
            .serve(
                amf_address.to_string(),
                stop_source.token(),
                Handler(internal_sender),
                logger.clone(),
            )
            .await
            .expect("Server bind failed");

        MockAmf {
            receiver,
            stop_source,
            sender: server,
            task,
            logger,
        }
    }

    pub async fn expect_connection(&self) {
        // Wait for connection to be established - the mock TNLA event handler sends us an empty message to indicate this.
        trace!(self.logger, "Wait for connection from worker");
        assert!(self
            .receiver
            .recv()
            .await
            .expect("Failed mock recv")
            .is_none());
    }

    async fn receive_ngap_pdu(&self) -> NgapPdu {
        self.receiver
            .recv()
            .await
            .expect("Expected message")
            .expect("Expected message")
    }

    pub async fn handle_ng_setup(&self, logger: &Logger) -> Result<()> {
        // Catch NG Setup from the GNB
        info!(logger, "Wait for NG Setup from GNB");

        let pdu = self.receive_ngap_pdu().await;

        if let NgapPdu::InitiatingMessage(InitiatingMessage::NgSetupRequest(_ng_setup)) = pdu {
            info!(self.logger, "Got NG Setup, send setup response");
            Ok(())
        } else {
            Err(anyhow!("Not an NG setup"))
        }?;

        //let amf_name = AmfName("MockAmf".to_string());
        let plmn_identity = PlmnIdentity(vec![0, 0, 1, 0, 1]);
        let served_guami_list = ServedGuamiList(vec![ServedGuamiItem {
            guami: Guami {
                plmn_identity,
                amf_region_id: AmfRegionId(BitVec::new()),
                amf_set_id: AmfSetId(BitVec::new()),
                amf_pointer: AmfPointer(BitVec::new()),
            },
            backup_amf_name: None,
        }]);

        let response =
            NgapPdu::SuccessfulOutcome(SuccessfulOutcome::NgSetupResponse(NgSetupResponse {
                amf_name: AmfName("MockAmf".to_string()),
                served_guami_list,
                relative_amf_capacity: RelativeAmfCapacity(100),
                plmn_support_list: PlmnSupportList(vec![]),
                criticality_diagnostics: None,
                ue_retention_information: None,
                iab_supported: None,
                extended_amf_name: None,
            }));

        self.sender.send_pdu(response, &logger).await?;

        Ok(())
    }

    pub async fn handle_ran_configuration_update(&self, logger: &Logger) -> Result<()> {
        info!(logger, "Wait for RAN Configuration Update from GNB");

        let pdu = self.receive_ngap_pdu().await;

        if let NgapPdu::InitiatingMessage(InitiatingMessage::RanConfigurationUpdate(
            _ran_configuration_update,
        )) = pdu
        {
            info!(logger, "Got RAN configuration update, send response");
            Ok(())
        } else {
            Err(anyhow!("Not a RAN configuration update"))
        }?;

        let response =
            NgapPdu::SuccessfulOutcome(SuccessfulOutcome::RanConfigurationUpdateAcknowledge(
                RanConfigurationUpdateAcknowledge {
                    criticality_diagnostics: None,
                },
            ));

        self.sender.send_pdu(response, &logger).await?;

        Ok(())
    }
}

#[async_trait]
impl TnlaEventHandler<NgapPdu> for Handler {
    async fn handle_event(&self, _event: TnlaEvent, _tnla_id: u32, _logger: &Logger) {
        self.0.send(None).await.unwrap();
    }

    // TODO indicate whether it is UE or non UE associated?
    async fn handle_message(&self, message: NgapPdu, _tnla_id: u32, logger: &Logger) {
        trace!(logger, "Got message from GNB");
        self.0.send(Some(message)).await.unwrap();
    }
}
