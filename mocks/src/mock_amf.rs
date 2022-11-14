//! mock_amf - enables a test script to assume the role of the AMF on the NG reference point

use crate::mock::{Mock, Pdu, ReceivedPdu};
use anyhow::{anyhow, Result};
use bitvec::prelude::*;
use net::AperSerde;
use ngap::*;
use slog::{debug, info, o, Logger};
use std::ops::Deref;

impl Pdu for NgapPdu {}

pub struct MockAmf {
    mock: Mock<NgapPdu>,
}

pub struct UeContext {
    ue_id: u32,
    ran_ue_ngap_id: RanUeNgapId,
}

impl Deref for MockAmf {
    type Target = Mock<NgapPdu>;

    fn deref(&self) -> &Self::Target {
        &self.mock
    }
}

const NGAP_SCTP_PPID: u32 = 60;
const NGAP_BIND_PORT: u16 = 38412;

impl MockAmf {
    pub async fn new(amf_ip_address: &str, logger: &Logger) -> Result<MockAmf> {
        let mut mock = Mock::new(logger.new(o!("amf" => 1))).await;
        let listen_address = format!("{}:{}", amf_ip_address, NGAP_BIND_PORT);
        info!(logger, "Mock AMF listening on {}", listen_address);
        mock.serve(listen_address, NGAP_SCTP_PPID).await?;
        Ok(MockAmf { mock })
    }

    pub async fn terminate(self) {
        self.mock.terminate().await
    }

    pub async fn handle_ng_setup(&self) -> Result<()> {
        let logger = &self.logger;
        info!(logger, "Wait for NG Setup from GNB");

        let ReceivedPdu { pdu, assoc_id } = self.receive_pdu_with_assoc_id().await;

        if let NgapPdu::InitiatingMessage(InitiatingMessage::NgSetupRequest(_ng_setup)) = pdu {
            info!(logger, ">> NgSetupRequest");
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

        info!(logger, "<< NgSetupResponse");
        self.send(response.into_bytes()?, Some(assoc_id)).await;

        Ok(())
    }

    pub async fn handle_ran_configuration_update(&self) -> Result<()> {
        let logger = &self.logger;
        info!(logger, "Wait for RAN Configuration Update from GNB");

        let ReceivedPdu { pdu, assoc_id } = self.receive_pdu_with_assoc_id().await;

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
        self.send(response.into_bytes()?, Some(assoc_id)).await;

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

    pub async fn receive_initial_ue_message(&self, ue_id: u32) -> Result<UeContext> {
        let logger = &self.logger;
        if let NgapPdu::InitiatingMessage(InitiatingMessage::InitialUeMessage(InitialUeMessage {
            ran_ue_ngap_id,
            ..
        })) = self.receive_pdu().await
        {
            info!(logger, ">> InitialUeMessage");
            debug!(logger, "UE Id {:?}", ran_ue_ngap_id);
            Ok(UeContext {
                ue_id,
                ran_ue_ngap_id,
            })
        } else {
            Err(anyhow!("Not an initial UE message"))
        }
    }

    pub async fn send_initial_context_setup_request(&self, ue_context: &UeContext) -> Result<()> {
        let logger = &self.logger;
        let pdu = NgapPdu::InitiatingMessage(InitiatingMessage::InitialContextSetupRequest(
            InitialContextSetupRequest {
                amf_ue_ngap_id: AmfUeNgapId(ue_context.ue_id.into()),
                ran_ue_ngap_id: ue_context.ran_ue_ngap_id,
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
        self.send(pdu.into_bytes()?, None).await;
        Ok(())
    }

    pub async fn receive_initial_context_setup_response(
        &self,
        ue_context: &UeContext,
    ) -> Result<()> {
        let received_amf_ue_id = match self.receive_pdu().await {
            NgapPdu::SuccessfulOutcome(SuccessfulOutcome::InitialContextSetupResponse(
                InitialContextSetupResponse { amf_ue_ngap_id, .. },
            )) => {
                info!(&self.logger, ">> InitialContextSetupResponse");
                Ok(amf_ue_ngap_id.0)
            }
            x => Err(anyhow!(
                "Expecting InitialContextSetupResponse, got unexpected message {:?}",
                x
            )),
        }?;
        assert_eq!(received_amf_ue_id, ue_context.ue_id.into());
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
        self.send(pdu.into_bytes()?, None).await;
        Ok(())
    }

    pub async fn send_pdu_session_resource_setup(&self, ue_context: &UeContext) -> Result<()> {
        info!(&self.logger, "<< PduSessionResourceSetupRequest");

        let transport_layer_address = TransportLayerAddress(bitvec![u8, Msb0;0,1,0,1,0,1]);

        let pdu_session_resource_setup_request_transfer = PduSessionResourceSetupRequestTransfer {
            pdu_session_aggregate_maximum_bit_rate: None,
            ul_ngu_up_tnl_information: UpTransportLayerInformation::GtpTunnel(GtpTunnel {
                transport_layer_address,
                gtp_teid: GtpTeid(vec![1, 2, 3, 4]),
            }),
            additional_ul_ngu_up_tnl_information: None,
            data_forwarding_not_possible: None,
            pdu_session_type: PduSessionType::Ipv4,
            security_indication: None,
            network_instance: None,
            qos_flow_setup_request_list: QosFlowSetupRequestList(vec![QosFlowSetupRequestItem {
                qos_flow_identifier: QosFlowIdentifier(1),
                qos_flow_level_qos_parameters: QosFlowLevelQosParameters {
                    qos_characteristics: QosCharacteristics::NonDynamic5qi(
                        NonDynamic5qiDescriptor {
                            five_qi: FiveQi(1),
                            priority_level_qos: None,
                            averaging_window: None,
                            maximum_data_burst_volume: None,
                        },
                    ),
                    allocation_and_retention_priority: AllocationAndRetentionPriority {
                        priority_level_arp: PriorityLevelArp(3),
                        pre_emption_capability: PreEmptionCapability::ShallNotTriggerPreEmption,
                        pre_emption_vulnerability: PreEmptionVulnerability::PreEmptable,
                    },
                    gbr_qos_information: None,
                    reflective_qos_attribute: None,
                    additional_qos_flow_information: None,
                },
                e_rab_id: None,
            }]),
            common_network_instance: None,
            direct_forwarding_path_availability: None,
            redundant_ul_ngu_up_tnl_information: None,
            additional_redundant_ul_ngu_up_tnl_information: None,
            redundant_common_network_instance: None,
            redundant_pdu_session_information: None,
        }
        .into_bytes()?;

        let pdu = NgapPdu::InitiatingMessage(InitiatingMessage::PduSessionResourceSetupRequest(
            PduSessionResourceSetupRequest {
                amf_ue_ngap_id: AmfUeNgapId(ue_context.ue_id.into()),
                ran_ue_ngap_id: ue_context.ran_ue_ngap_id,
                ran_paging_priority: None,
                nas_pdu: None,
                pdu_session_resource_setup_list_su_req: PduSessionResourceSetupListSuReq(vec![
                    PduSessionResourceSetupItemSuReq {
                        pdu_session_id: PduSessionId(1),
                        pdu_session_nas_pdu: Some(NasPdu(vec![])),
                        s_nssai: self.snssai(),
                        pdu_session_resource_setup_request_transfer,
                    },
                ]),
                ue_aggregate_maximum_bit_rate: None,
            },
        ));
        self.send(pdu.into_bytes()?, None).await;
        Ok(())
    }

    pub async fn receive_pdu_session_resource_setup_response(
        &self,
        ue_context: &UeContext,
    ) -> Result<()> {
        match self.receive_pdu().await {
            NgapPdu::SuccessfulOutcome(SuccessfulOutcome::PduSessionResourceSetupResponse(
                PduSessionResourceSetupResponse {
                    amf_ue_ngap_id,
                    pdu_session_resource_setup_list_su_res,
                    ..
                },
            )) => {
                info!(&self.logger, ">> PduSessionResourceSetupResponse");
                assert_eq!(amf_ue_ngap_id.0, ue_context.ue_id.into());
                if let Some(xs) = pdu_session_resource_setup_list_su_res {
                    // There should be exactly one successful session setup.
                    assert!(xs.0.len() == 1);
                } else {
                    panic!("Expected pdu_session_resource_setup_list_su_res on PduSessionResourceSetupResponse")
                }
                Ok(())
            }
            x => panic!(
                "Expecting PduSessionResourceSetupResponse, got unexpected message {:?}",
                x
            ),
        }
    }
}
