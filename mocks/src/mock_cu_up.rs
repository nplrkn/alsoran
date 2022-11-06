//! mock_cu_up - enables a test script to assume the role of the GNB-CU-UP on the E1 reference point

use crate::mock::{Mock, Pdu};
use anyhow::{anyhow, Result};
use bitvec::prelude::*;
use e1ap::*;
use net::AperSerde;
use slog::{debug, info, o, Logger};
use std::{
    collections::HashMap,
    net::IpAddr,
    ops::{Deref, DerefMut},
};

impl Pdu for E1apPdu {}

const E1AP_SCTP_PPID: u32 = 64;

pub struct MockCuUp {
    mock: Mock<E1apPdu>,
    ues: HashMap<u32, UeContext>,
}

struct UeContext {
    gnb_cu_cp_ue_e1ap_id: GnbCuCpUeE1apId,
}

impl Deref for MockCuUp {
    type Target = Mock<E1apPdu>;

    fn deref(&self) -> &Self::Target {
        &self.mock
    }
}

impl DerefMut for MockCuUp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mock
    }
}

impl MockCuUp {
    pub async fn new(logger: &Logger) -> MockCuUp {
        let logger = logger.new(o!("cuup" => 1));
        let mock = Mock::new(logger).await;
        MockCuUp {
            mock,
            ues: HashMap::new(),
        }
    }

    pub async fn terminate(self) {
        self.mock.terminate().await
    }

    pub async fn perform_e1_setup(&self) -> Result<()> {
        self.send_e1_setup_request().await?;
        self.receive_e1_setup_response().await;
        Ok(())
    }

    async fn send_e1_setup_request(&self) -> Result<()> {
        let supported_plmns = SupportedPlmnsList(vec![SupportedPlmnsItem {
            plmn_identity: PlmnIdentity(vec![0, 1, 2]),
            slice_support_list: None,
            nr_cgi_support_list: None,
            qos_parameters_support_list: None,
        }]);
        let pdu = e1ap::E1apPdu::InitiatingMessage(InitiatingMessage::GnbCuUpE1SetupRequest(
            GnbCuUpE1SetupRequest {
                transaction_id: TransactionId(0),
                gnb_cu_up_id: GnbCuUpId(232),
                gnb_cu_up_name: Some(GnbCuUpName("TestCuUp".to_string())),
                cn_support: CnSupport::C5gc,
                supported_plmns,
                gnb_cu_up_capacity: None,
                transport_layer_address_info: None,
                extended_gnb_cu_up_name: None,
            },
        ));
        info!(self.logger, "E1SetupRequest >>");
        self.send(pdu.into_bytes()?).await;
        Ok(())
    }

    async fn receive_e1_setup_response(&self) {
        let _response = self.receive_pdu().await;
        info!(self.logger, "E1SetupResponse <<");
    }

    pub async fn handle_cu_cp_configuration_update(
        &mut self,
        expected_addr_string: &String,
    ) -> Result<()> {
        let expected_address = convert_transport_address(expected_addr_string)?;
        let transaction_id = self
            .receive_cu_cp_configuration_update(&expected_address)
            .await?;
        // connect
        self.connect(&expected_addr_string, E1AP_SCTP_PPID).await;
        self.send_cu_cp_configuration_update_acknowledge(transaction_id, expected_address)
            .await
    }

    async fn receive_cu_cp_configuration_update(
        &self,
        expected_address: &TransportLayerAddress,
    ) -> Result<TransactionId> {
        let cu_cp_configuration_update = match self.receive_pdu().await {
            E1apPdu::InitiatingMessage(InitiatingMessage::GnbCuCpConfigurationUpdate(x)) => Ok(x),
            x => Err(anyhow!("Expected GnbCuCpConfigurationUpdate, got {:?}", x)),
        }?;
        info!(self.logger, "GnbCuCpConfigurationUpdate <<");

        let gnb_cu_cp_tnla_to_add_list = cu_cp_configuration_update
            .gnb_cu_cp_tnla_to_add_list
            .expect("Expected gnb_cu_cp_tnla_to_add_list to be present");
        match &gnb_cu_cp_tnla_to_add_list
            .0
            .first()
            .expect("Expected nonempty gnb_cu_cp_tnla_to_add_list list")
            .tnl_association_transport_layer_address
        {
            CpTnlInformation::EndpointIpAddress(ref x) => {
                assert_eq!(x.0, expected_address.0);
            }
        };

        Ok(cu_cp_configuration_update.transaction_id)
    }

    async fn send_cu_cp_configuration_update_acknowledge(
        &self,
        transaction_id: TransactionId,
        transport_layer_address: TransportLayerAddress,
    ) -> Result<()> {
        let pdu = e1ap::E1apPdu::SuccessfulOutcome(
            SuccessfulOutcome::GnbCuCpConfigurationUpdateAcknowledge(
                GnbCuCpConfigurationUpdateAcknowledge {
                    transaction_id,
                    criticality_diagnostics: None,
                    gnb_cu_cp_tnla_setup_list: Some(GnbCuCpTnlaSetupList(vec![
                        GnbCuCpTnlaSetupItem {
                            tnl_association_transport_layer_address:
                                CpTnlInformation::EndpointIpAddress(transport_layer_address),
                        },
                    ])),
                    gnb_cu_cp_tnla_failed_to_setup_list: None,
                    transport_layer_address_info: None,
                },
            ),
        );

        info!(self.logger, "GnbCuCpConfigurationUpdateAcknowledge >>");
        self.send(pdu.into_bytes()?).await;
        Ok(())
    }

    pub async fn handle_bearer_context_setup(&mut self, ue_id: u32) -> Result<()> {
        self.receive_bearer_context_setup(ue_id).await?;
        self.send_bearer_context_setup_response(ue_id).await
    }

    async fn receive_bearer_context_setup(&mut self, ue_id: u32) -> Result<()> {
        let logger = &self.logger;

        let bearer_context_setup = match self.receive_pdu().await {
            E1apPdu::InitiatingMessage(InitiatingMessage::BearerContextSetupRequest(x)) => Ok(x),
            x => Err(anyhow!("Expected BearerContextSetupRequest, got {:?}", x)),
        }?;
        let gnb_cu_cp_ue_e1ap_id = bearer_context_setup.gnb_cu_cp_ue_e1ap_id;
        debug!(logger, "UE Id {:?}", gnb_cu_cp_ue_e1ap_id);
        self.ues.insert(
            ue_id,
            UeContext {
                gnb_cu_cp_ue_e1ap_id,
            },
        );
        info!(self.logger, "BearerContextSetupRequest <<");
        Ok(())
    }

    async fn send_bearer_context_setup_response(&self, ue_id: u32) -> Result<()> {
        let gnb_cu_cp_ue_e1ap_id = self.ues[&ue_id].gnb_cu_cp_ue_e1ap_id.clone();

        let upf_facing_transport_layer_address = bitvec![u8, Msb0;1, 0, 1, 0,1,0];

        let du_facing_transport_layer_address = bitvec![u8, Msb0;1, 1, 1, 0,0,0];

        let pdu = e1ap::E1apPdu::SuccessfulOutcome(SuccessfulOutcome::BearerContextSetupResponse(
            BearerContextSetupResponse {
                gnb_cu_cp_ue_e1ap_id,
                gnb_cu_up_ue_e1ap_id: GnbCuUpUeE1apId(ue_id),
                system_bearer_context_setup_response:
                    SystemBearerContextSetupResponse::NgRanBearerContextSetupResponse(
                        NgRanBearerContextSetupResponse {
                            pdu_session_resource_setup_list: PduSessionResourceSetupList(vec![
                                PduSessionResourceSetupItem {
                                    pdu_session_id: PduSessionId(1),
                                    security_result: None,
                                    ng_dl_up_tnl_information: UpTnlInformation::GtpTunnel(
                                        GtpTunnel {
                                            transport_layer_address: TransportLayerAddress(
                                                upf_facing_transport_layer_address,
                                            ),
                                            gtp_teid: GtpTeid(vec![2, 3, 2, 1]),
                                        },
                                    ),
                                    pdu_session_data_forwarding_information_response: None,
                                    ng_dl_up_unchanged: None,
                                    drb_setup_list_ng_ran: DrbSetupListNgRan(vec![
                                        DrbSetupItemNgRan {
                                            drb_id: DrbId(1),
                                            drb_data_forwarding_information_response: None,
                                            ul_up_transport_parameters: UpParameters(vec![
                                                UpParametersItem {
                                                    up_tnl_information: UpTnlInformation::GtpTunnel(
                                                        GtpTunnel {
                                                            transport_layer_address:
                                                                TransportLayerAddress(du_facing_transport_layer_address),
                                                            gtp_teid: GtpTeid(vec![2, 3, 2, 1]),
                                                        },
                                                    ),
                                                    cell_group_id: CellGroupId(1),
                                                },
                                            ]),
                                            flow_setup_list: QosFlowList(vec![QosFlowItem {
                                                qos_flow_identifier: QosFlowIdentifier(1),
                                            }]),
                                            flow_failed_list: None,
                                        },
                                    ]),
                                    drb_failed_list_ng_ran: None,
                                },
                            ]),
                            pdu_session_resource_failed_list: None,
                        },
                    ),
            },
        ));
        info!(self.logger, "BearerContextSetupResponse >>");
        self.send(pdu.into_bytes()?).await;
        Ok(())
    }

    pub async fn handle_bearer_context_modification(&self, ue_id: u32) -> Result<()> {
        self.receive_bearer_context_modification(ue_id).await?;
        self.send_bearer_context_modification_response(ue_id).await
    }

    async fn receive_bearer_context_modification(&self, ue_id: u32) -> Result<()> {
        let bearer_context_modification = match self.receive_pdu().await {
            E1apPdu::InitiatingMessage(InitiatingMessage::BearerContextModificationRequest(x)) => {
                Ok(x)
            }
            x => Err(anyhow!(
                "Expected BearerContextModificationRequest, got {:?}",
                x
            )),
        }?;
        info!(self.logger, "BearerContextModificationRequest <<");

        // Check the IDs.
        let ue = &self.ues[&ue_id];
        assert_eq!(bearer_context_modification.gnb_cu_up_ue_e1ap_id.0, ue_id);
        assert_eq!(
            bearer_context_modification.gnb_cu_cp_ue_e1ap_id.0,
            ue.gnb_cu_cp_ue_e1ap_id.0
        );
        Ok(())
    }

    async fn send_bearer_context_modification_response(&self, ue_id: u32) -> Result<()> {
        let ue = &self.ues[&ue_id];
        let pdu = e1ap::E1apPdu::SuccessfulOutcome(SuccessfulOutcome::BearerContextModificationResponse(BearerContextModificationResponse {
            gnb_cu_cp_ue_e1ap_id: ue.gnb_cu_cp_ue_e1ap_id,
            gnb_cu_up_ue_e1ap_id: GnbCuUpUeE1apId(ue_id),
            system_bearer_context_modification_response: Some(
                SystemBearerContextModificationResponse::NgRanBearerContextModificationResponse(
                    NgRanBearerContextModificationResponse {
                        // TODO - supply these to make a proper reply to the request
                        pdu_session_resource_setup_mod_list: None,
                        pdu_session_resource_failed_mod_list: None,
                        pdu_session_resource_modified_list: None,
                        pdu_session_resource_failed_to_modify_list: None,
                        retainability_measurements_info: None,
                    },
                ),
            ),
        }));
        info!(self.logger, "BearerContextModificationResponse >>");
        self.send(pdu.into_bytes()?).await;

        Ok(())
    }
}

fn convert_transport_address(addr: &String) -> Result<TransportLayerAddress> {
    Ok(TransportLayerAddress(match addr.parse()? {
        IpAddr::V4(x) => BitVec::<_, Msb0>::from_slice(&x.octets()),
        IpAddr::V6(x) => BitVec::<_, Msb0>::from_slice(&x.octets()),
    }))
}
