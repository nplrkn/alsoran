use crate::mock::{Mock, Pdu};
use anyhow::Result;
use e1ap::*;
use net::AperSerde;
use slog::{info, o, Logger};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

impl Pdu for E1apPdu {}

pub struct MockCuUp {
    mock: Mock<E1apPdu>,
    _ues: HashMap<u32, UeContext>,
}

struct UeContext {}

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
            _ues: HashMap::new(),
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
        let pdu = e1ap::E1apPdu::InitiatingMessage(InitiatingMessage::GnbCuUpE1SetupRequest(
            GnbCuUpE1SetupRequest {
                transaction_id: TransactionId(0),
                gnb_cu_up_id: GnbCuUpId(232),
                gnb_cu_up_name: Some(GnbCuUpName("TestCuUp".to_string())),
                cn_support: CnSupport::C5gc,
                supported_plmns: SupportedPlmnsList(vec![]),
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
}
