use anyhow::{anyhow, Result};
use async_channel::{Receiver, Sender};
use async_std::task::JoinHandle;
use async_trait::async_trait;
use bitvec::prelude::*;
use net::{AperSerde, SctpTransportProvider, TnlaEvent, TnlaEventHandler, TransportProvider};
use ngap::*;
use slog::{debug, info, o, trace, Logger};
use std::fmt::Debug;
use stop_token::StopSource;

const NGAP_SCTP_PPID: u32 = 60;

// TODO why pub?
pub struct MockAmf {
    pub stop_source: StopSource,
    pub receiver: Receiver<Option<NgapPdu>>,
    pub sender: SctpTransportProvider,
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
        let server = SctpTransportProvider::new(NGAP_SCTP_PPID);
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
        debug!(self.logger, "Wait for connection from worker");
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

    pub async fn handle_ng_setup(&self) -> Result<()> {
        // Catch NG Setup from the GNB
        let logger = &self.logger;
        info!(logger, "Wait for NG Setup from GNB");

        let pdu = self.receive_ngap_pdu().await;

        if let NgapPdu::InitiatingMessage(InitiatingMessage::NgSetupRequest(_ng_setup)) = pdu {
            info!(self.logger, "Got NG Setup, send setup response");
            Ok(())
        } else {
            Err(anyhow!("Not an NG setup"))
        }?;

        let served_guami_list = ServedGuamiList(vec![ServedGuamiItem {
            guami: self.guami(),
            backup_amf_name: None,
        }]);

        let response =
            NgapPdu::SuccessfulOutcome(SuccessfulOutcome::NgSetupResponse(NgSetupResponse {
                amf_name: AmfName("MockAmf".to_string()),
                served_guami_list,
                relative_amf_capacity: RelativeAmfCapacity(100),
                plmn_support_list: PlmnSupportList(vec![PlmnSupportItem {
                    plmn_identity: self.plmn_identity(),
                    slice_support_list: SliceSupportList(vec![SliceSupportItem {
                        s_nssai: self.snssai(),
                    }]),
                }]),
                criticality_diagnostics: None,
                ue_retention_information: None,
                iab_supported: None,
                extended_amf_name: None,
            }));

        self.sender
            .send_message(response.into_bytes()?, &logger)
            .await?;

        Ok(())
    }

    fn guami(&self) -> Guami {
        Guami {
            plmn_identity: self.plmn_identity(),
            amf_region_id: AmfRegionId(bitvec![Msb0,u8;1;8]),
            amf_set_id: AmfSetId(bitvec![Msb0,u8;1;10]),
            amf_pointer: AmfPointer(bitvec![Msb0,u8;1;6]),
        }
    }

    fn plmn_identity(&self) -> PlmnIdentity {
        PlmnIdentity(vec![2, 3, 2])
    }

    fn snssai(&self) -> SNssai {
        SNssai {
            sst: Sst(vec![0x01]),
            sd: None,
        }
    }

    pub async fn handle_ran_configuration_update(&self, logger: &Logger) -> Result<()> {
        info!(logger, "Wait for RAN Configuration Update from GNB");

        let pdu = self.receive_ngap_pdu().await;

        if let NgapPdu::InitiatingMessage(InitiatingMessage::RanConfigurationUpdate(
            _ran_configuration_update,
        )) = pdu
        {
            info!(logger, ">> RanConfigurationUpdate");
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

        info!(logger, "<< RanConfigurationUpdateAcknowledge");
        self.sender
            .send_message(response.into_bytes()?, &logger)
            .await?;

        Ok(())
    }

    pub async fn receive_initial_ue_message(&self) -> Result<()> {
        let logger = &self.logger;
        if let NgapPdu::InitiatingMessage(InitiatingMessage::InitialUeMessage(
            _initial_ue_message,
        )) = self.receive_ngap_pdu().await
        {
            info!(logger, ">> InitialUeMessage");
            Ok(())
        } else {
            Err(anyhow!("Not an initial UE message"))
        }
    }

    pub async fn send_initial_context_setup_request(&self) -> Result<()> {
        let logger = &self.logger;
        let pdu = NgapPdu::InitiatingMessage(InitiatingMessage::InitialContextSetupRequest(
            InitialContextSetupRequest {
                amf_ue_ngap_id: AmfUeNgapId(1),
                ran_ue_ngap_id: RanUeNgapId(1),
                old_amf: None,
                ue_aggregate_maximum_bit_rate: None,
                core_network_assistance_information_for_inactive: None,
                guami: self.guami(),
                pdu_session_resource_setup_list_cxt_req: None,
                allowed_nssai: AllowedNssai(vec![AllowedNssaiItem {
                    s_nssai: self.snssai(),
                }]),
                ue_security_capabilities: UeSecurityCapabilities {
                    nr_encryption_algorithms: NrEncryptionAlgorithms(bitvec![Msb0,u8;0;16]),
                    nr_integrity_protection_algorithms: NrIntegrityProtectionAlgorithms(
                        bitvec![Msb0,u8;0;16],
                    ),
                    eutr_aencryption_algorithms: EutrAencryptionAlgorithms(bitvec![Msb0,u8;0;16]),
                    eutr_aintegrity_protection_algorithms: EutrAintegrityProtectionAlgorithms(
                        bitvec![Msb0,u8;0;16],
                    ),
                },
                security_key: SecurityKey(bitvec![Msb0,u8;0;256]),
                trace_activation: None,
                mobility_restriction_list: None,
                ue_radio_capability: None,
                index_to_rfsp: None,
                masked_imeisv: None,
                nas_pdu: None,
                emergency_fallback_indicator: None,
                rrc_inactive_transition_report_request: None,
                ue_radio_capability_for_paging: None,
                redirection_voice_fallback: None,
                location_reporting_request_type: None,
                cn_assisted_ran_tuning: None,
                srvcc_operation_possible: None,
                iab_authorized: None,
                enhanced_coverage_restriction: None,
                extended_connected_time: None,
                ue_differentiation_info: None,
                nr_v2x_services_authorized: None,
                ltev2x_services_authorized: None,
                nr_ue_sidelink_aggregate_maximum_bitrate: None,
                lte_ue_sidelink_aggregate_maximum_bitrate: None,
                pc5_qos_parameters: None,
                c_emode_brestricted: None,
                ue_up_c_iot_support: None,
                rg_level_wireline_access_characteristics: None,
                management_based_mdt_plmn_list: None,
                ue_radio_capability_id: None,
            },
        ));

        info!(logger, "<< InitialContextSetupRequest");
        self.sender.send_message(pdu.into_bytes()?, &logger).await
    }

    pub async fn receive_initial_context_setup_response(&self) -> Result<()> {
        todo!()
    }
}

#[async_trait]
impl TnlaEventHandler for Handler {
    async fn handle_event(&self, _event: TnlaEvent, _tnla_id: u32, _logger: &Logger) {
        self.0.send(None).await.unwrap();
    }

    // TODO indicate whether it is UE or non UE associated?
    async fn handle_message(
        &self,
        message: Vec<u8>,
        _tnla_id: u32,
        logger: &Logger,
    ) -> Option<Vec<u8>> {
        trace!(logger, "Got message from GNB");
        self.0
            .send(Some(NgapPdu::from_bytes(&message).unwrap()))
            .await
            .unwrap();
        None
    }
}
