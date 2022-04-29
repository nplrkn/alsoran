use super::Gnbcu;
use anyhow::Result;
use async_trait::async_trait;
use net::Message;
use net::{Application, TnlaEvent};
use ngap::{InitiatingMessage, NgapGnbProvider};
use ngap::{NgapPdu, RanConfigurationUpdateProcedure};
use slog::{trace, warn, Logger};
use xxap_transaction::{AperCodec, RequestError, RequestProvider};

#[derive(Clone)]
pub struct NgapHandler {
    gnbcu: Gnbcu,
}

impl NgapHandler {
    pub fn new(gnbcu: Gnbcu) -> Self {
        Self { gnbcu }
    }
}

// Should split this into EventHandler and RequestIndicationHandler.
// This is an improvement on Application since it can be automatically implemented

#[async_trait]
trait RequestHandler {
    async fn handle_request(
        &self,
        message: Message,
        _tnla_id: u32,
        logger: &Logger,
    ) -> Result<Message>;
}

#[async_trait]
impl<T: NgapGnbProvider + Send + Sync> RequestHandler for T {
    async fn handle_request(
        &self,
        message: Message,
        _tnla_id: u32,
        logger: &Logger,
    ) -> Result<Message> {
        let pdu = <Self as NgapGnbProvider>::route_request(
            self,
            InitiatingMessage::from_bytes(&message).unwrap(), // TODO
            logger,
        )
        .await?;
        Ok(pdu.into_bytes()?)
    }
}

#[async_trait]
impl Application for NgapHandler {
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        match event {
            TnlaEvent::Established => trace!(logger, "NGAP TNLA {} established", tnla_id),
            TnlaEvent::Terminated => warn!(logger, "NGAP TNLA {} closed", tnla_id),
        };
        self.gnbcu.connected_amf_change(logger).await;
    }

    async fn handle_request(
        &self,
        message: Message,
        _tnla_id: u32,
        logger: &Logger,
    ) -> Result<Message> {
        let pdu = <Self as NgapGnbProvider>::route_request(
            self,
            InitiatingMessage::from_bytes(&message).unwrap(), // TODO
            logger,
        )
        .await?;
        Ok(pdu.into_bytes()?)
    }
}

#[async_trait]
impl NgapGnbProvider for NgapHandler {}

// This uses the default implemented which is to retun an error of unimplemented
impl RequestProvider<RanConfigurationUpdateProcedure> for NgapHandler {}
