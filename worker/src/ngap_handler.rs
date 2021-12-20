use crate::gnbcu::Gnbcu;
use crate::ClientContext;
use async_trait::async_trait;
use common::transport_provider::{ClientTransportProvider, Handler, Message, TransportProvider};
use node_control_api::Api;
use slog::Logger;
use slog::{info, warn};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct NgapHandler<T, F, C>
where
    T: ClientTransportProvider,
    F: TransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
{
    gnbcu: Arc<Gnbcu<T, F, C>>,
}

impl<T, F, C> NgapHandler<T, F, C>
where
    T: ClientTransportProvider,
    F: TransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
{
    pub fn new(gnbcu: Gnbcu<T, F, C>) -> NgapHandler<T, F, C> {
        NgapHandler {
            gnbcu: Arc::new(gnbcu),
        }
    }
}

use asn1_codecs::aper::{AperCodec, AperCodecData};
use common::ngap::NGAP_PDU;

#[async_trait]
impl<T, F, C> Handler for NgapHandler<T, F, C>
where
    T: ClientTransportProvider,
    F: TransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
{
    async fn recv_non_ue_associated(&self, message: Message, logger: &Logger) {
        // info!(
        //     logger,
        //     "NgapHandler got non UE associated message {:?}", message
        // );
        let mut codec_data = AperCodecData::from_slice(&message);

        match NGAP_PDU::decode(&mut codec_data) {
            Ok(ngap_pdu) => info!(logger, "ngap_pdu: {:#?}", ngap_pdu),
            Err(e) => warn!(logger, "ngap decode failure {:?}", e),
        };
        // self.gnbcu
        //     .f1_transport_provider
        //     .send_message(message, &logger)
        //     .await
        //     .unwrap();
    }
}
