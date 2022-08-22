use crate::mock::{Mock, Pdu};
use anyhow::{anyhow, Result};
use bitvec::prelude::*;
use net::AperSerde;
use ngap::*;
use slog::{debug, info, o, Logger};
use std::{collections::HashMap, ops::Deref};

impl Pdu for NgapPdu {}

pub struct MockAmf {
    mock: Mock<NgapPdu>,
    ues: HashMap<u32, UeContext>,
}

struct UeContext {
    ran_ue_ngap_id: RanUeNgapId,
}

impl Deref for MockAmf {
    type Target = Mock<NgapPdu>;

    fn deref(&self) -> &Self::Target {
        &self.mock
    }
}

const NGAP_SCTP_PPID: u32 = 60;

impl MockAmf {
    pub async fn new(amf_address: &str, logger: &Logger) -> MockAmf {
        let mut mock = Mock::new(logger.new(o!("amf" => 1)), NGAP_SCTP_PPID).await;
        mock.serve(amf_address.to_string()).await;
        MockAmf {
            mock,
            ues: HashMap::new(),
        }
    }

    pub async fn terminate(self) {
        self.mock.terminate().await
    }

    pub async fn handle_ng_setup(&self) -> Result<()> {
        let logger = &self.logger;
        info!(logger, "Wait for NG Setup from GNB");

        let pdu = self.receive_pdu().await;

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

        self.send(response.into_bytes()?).await;

        Ok(())
    }

    fn guami(&self) -> Guami {
        Guami {
            plmn_identity: self.plmn_identity(),
            amf_region_id: AmfRegionId(bitvec![u8,Msb0;1;8]),
            amf_set_id: AmfSetId(bitvec![u8,Msb0;1;10]),
            amf_pointer: AmfPointer(bitvec![u8,Msb0;1;6]),
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
        debug!(logger, "Wait for RAN Configuration Update from GNB");

        let pdu = self.receive_pdu().await;

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
        self.send(response.into_bytes()?).await;

        Ok(())
    }

    pub async fn receive_initial_ue_message(&mut self, ue_id: u32) -> Result<()> {
        let logger = &self.logger;
        if let NgapPdu::InitiatingMessage(InitiatingMessage::InitialUeMessage(InitialUeMessage {
            ran_ue_ngap_id,
            ..
        })) = self.receive_pdu().await
        {
            info!(logger, ">> InitialUeMessage");
            debug!(logger, "UE Id {:?}", ran_ue_ngap_id);
            self.ues.insert(ue_id, UeContext { ran_ue_ngap_id });
            Ok(())
        } else {
            Err(anyhow!("Not an initial UE message"))
        }
    }

    pub async fn send_initial_context_setup_request(&self, ue_id: u32) -> Result<()> {
        let logger = &self.logger;
        let ran_ue_ngap_id = self.ues[&ue_id].ran_ue_ngap_id.clone();
        let pdu = NgapPdu::InitiatingMessage(InitiatingMessage::InitialContextSetupRequest(
            InitialContextSetupRequest {
                amf_ue_ngap_id: AmfUeNgapId(ue_id.into()),
                ran_ue_ngap_id,
                old_amf: None,
                ue_aggregate_maximum_bit_rate: None,
                core_network_assistance_information_for_inactive: None,
                guami: self.guami(),
                pdu_session_resource_setup_list_cxt_req: None,
                allowed_nssai: AllowedNssai(vec![AllowedNssaiItem {
                    s_nssai: self.snssai(),
                }]),
                ue_security_capabilities: UeSecurityCapabilities {
                    nr_encryption_algorithms: NrEncryptionAlgorithms(bitvec![u8,Msb0;0;16]),
                    nr_integrity_protection_algorithms: NrIntegrityProtectionAlgorithms(
                        bitvec![u8,Msb0;0;16],
                    ),
                    eutr_aencryption_algorithms: EutrAencryptionAlgorithms(bitvec![u8,Msb0;0;16]),
                    eutr_aintegrity_protection_algorithms: EutrAintegrityProtectionAlgorithms(
                        bitvec![u8,Msb0;0;16],
                    ),
                },
                security_key: SecurityKey(bitvec![u8,Msb0;0;256]),
                trace_activation: None,
                mobility_restriction_list: None,
                ue_radio_capability: None,
                index_to_rfsp: None,
                masked_imeisv: None,
                nas_pdu: Some(NasPdu(Vec::new())),
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
        self.send(pdu.into_bytes()?).await;
        Ok(())
    }

    pub async fn receive_initial_context_setup_response(&self, ue_id: u32) -> Result<()> {
        info!(&self.logger, ">> InitialContextSetupResponse");
        let received_amf_ue_id = match self.receive_pdu().await {
            NgapPdu::SuccessfulOutcome(SuccessfulOutcome::InitialContextSetupResponse(
                InitialContextSetupResponse { amf_ue_ngap_id, .. },
            )) => Ok(amf_ue_ngap_id.0),
            x => Err(anyhow!(
                "Expecting InitialContextSetupResponse, got unexpected message {:?}",
                x
            )),
        }?;
        assert_eq!(received_amf_ue_id, ue_id.into());
        Ok(())
    }

    pub async fn send_status_indication(&self) -> Result<()> {
        info!(&self.logger, "<< AmfStatusIndication");
        let pdu = NgapPdu::InitiatingMessage(InitiatingMessage::AmfStatusIndication(
            AmfStatusIndication {
                unavailable_guami_list: UnavailableGuamiList(vec![UnavailableGuamiItem {
                    guami: self.guami(),
                    timer_approach_for_guami_removal: None,
                    backup_amf_name: None,
                }]),
            },
        ));
        self.send(pdu.into_bytes()?).await;
        Ok(())
    }
}
