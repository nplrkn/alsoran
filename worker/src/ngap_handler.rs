use crate::gnbcu::Gnbcu;
use crate::{ClientContext, F1ServerTransportProvider, NgapClientTransportProvider};
use also_net::{TnlaEvent, TnlaEventHandler};
use async_trait::async_trait;
use bitvec::vec::BitVec;
use common::ngap::*;
use node_control_api::Api;
use slog::Logger;
use slog::{info, trace, warn};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct NgapHandler<T, F, C>
where
    T: NgapClientTransportProvider,
    F: F1ServerTransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
{
    gnbcu: Arc<Gnbcu<T, F, C>>,
}

impl<T, F, C> NgapHandler<T, F, C>
where
    T: NgapClientTransportProvider,
    F: F1ServerTransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
{
    pub fn new(gnbcu: Gnbcu<T, F, C>) -> NgapHandler<T, F, C> {
        NgapHandler {
            gnbcu: Arc::new(gnbcu),
        }
    }
}

#[async_trait]
impl<T, F, C> TnlaEventHandler for NgapHandler<T, F, C>
where
    T: NgapClientTransportProvider,
    F: F1ServerTransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
{
    type MessageType = NgapPdu;

    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        match event {
            TnlaEvent::Established => {
                trace!(logger, "TNLA {} established", tnla_id);

                info!(logger, "Send NG setup to AMF");
                let ng_setup = NgapPdu::InitiatingMessage(InitiatingMessage {
                    procedure_code: ProcedureCode(21),
                    criticality: Criticality(Criticality::REJECT),
                    value: InitiatingMessageValue::IdNgSetup(NgSetupRequest {
                        protocol_i_es: NgSetupRequestProtocolIEs(vec![
                            NgSetupRequestProtocolIEsItem {
                                id: ProtocolIeId(27),
                                criticality: Criticality(Criticality::REJECT),
                                value: NgSetupRequestProtocolIEsItemValue::IdGlobalRanNodeId(
                                    GlobalRanNodeId::GlobalGnbId(GlobalGnbId {
                                        plmn_identity: PlmnIdentity(vec![2, 3, 2, 1, 5, 6]),
                                        gnb_id: GnbId::GnbId(BitString26(BitVec::from_element(
                                            0x10,
                                        ))),
                                        ie_extensions: None,
                                    }),
                                ),
                            },
                        ]),
                    }),
                });
                //let precanned_ng_setup = hex::decode("00150035000004001b00080002f83910000102005240090300667265653567630066001000000000010002f839000010080102030015400140").unwrap();
                match self
                    .gnbcu
                    .ngap_transport_provider
                    .send_pdu(ng_setup, logger)
                    .await
                {
                    Ok(()) => (),
                    Err(e) => warn!(logger, "Failed NG Setup send - {:?}", e),
                }
            }
            TnlaEvent::Terminated => warn!(logger, "TNLA {} closed", tnla_id),
        };
    }

    async fn handle_message(&self, message: NgapPdu, _tnla_id: u32, logger: &Logger) {
        info!(logger, "ngap_pdu: {:?}", message);
    }
}
