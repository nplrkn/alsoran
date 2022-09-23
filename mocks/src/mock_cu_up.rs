//! mock_cu_up - enables a test script to assume the role of the GNB-CU-UP on the E1 reference point

use crate::mock::{Mock, Pdu};
use anyhow::{anyhow, Result};
use bitvec::prelude::*;
use e1ap::*;
use net::AperSerde;
use slog::{debug, info, o, Logger};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

impl Pdu for E1apPdu {}

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

    pub async fn handle_bearer_context_modification(&self, _ue_id: u32) -> Result<()> {
        todo!()
    }
}
