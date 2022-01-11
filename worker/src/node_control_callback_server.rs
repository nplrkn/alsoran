//! Main library entry point for node_control_api implementation.
use crate::gnbcu::Gnbcu;
use crate::{ClientContext, F1ServerTransportProvider, NgapClientTransportProvider};
use async_trait::async_trait;
use bitvec::vec::BitVec;
use common::ngap::*;
use node_control_api::{models, Api, CallbackApi, TriggerInterfaceManagementResponse};
use slog::{info, warn};
use swagger::ApiError;

#[async_trait]
impl<T, F, C, Cx> CallbackApi<Cx> for Gnbcu<T, F, C>
where
    T: NgapClientTransportProvider,
    F: F1ServerTransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
    Cx: Send + Sync,
{
    /// A worker is instructed to trigger an interface management procedure on the given TNLA.
    async fn trigger_interface_management(
        &self,
        _callback_request_body_callback_url: String,
        _tnla_id: i32,
        interface_management_req: models::InterfaceManagementReq,
        _context: &Cx,
    ) -> Result<TriggerInterfaceManagementResponse, ApiError> {
        let logger = &self.logger;

        // TODO if a worker has max 1 connection to an AMF address, we don't need the TNLA ID.
        // We just need the AMF endpoint address.  Let's change the above to no longer ref tnla ID.

        // TODO check procedure and send RAN configuration update.
        info!(logger, "Send NG setup to AMF");
        let ng_setup = NgapPdu::InitiatingMessage(InitiatingMessage {
            procedure_code: ProcedureCode(21),
            criticality: Criticality(Criticality::REJECT),
            value: InitiatingMessageValue::IdNgSetup(NgSetupRequest {
                protocol_i_es: NgSetupRequestProtocolIEs(vec![NgSetupRequestProtocolIEsItem {
                    id: ProtocolIeId(27),
                    criticality: Criticality(Criticality::REJECT),
                    value: NgSetupRequestProtocolIEsItemValue::IdGlobalRanNodeId(
                        GlobalRanNodeId::GlobalGnbId(GlobalGnbId {
                            plmn_identity: PlmnIdentity(vec![2, 3, 2, 1, 5, 6]),
                            gnb_id: GnbId::GnbId(BitString26(BitVec::from_element(0x10))),
                            ie_extensions: None,
                        }),
                    ),
                }]),
            }),
        });
        //let precanned_ng_setup = hex::decode("00150035000004001b00080002f83910000102005240090300667265653567630066001000000000010002f839000010080102030015400140").unwrap();
        // TODO use tnla_id
        match self
            .ngap_transport_provider
            .send_pdu(ng_setup, logger)
            .await
        {
            Ok(()) => Ok(TriggerInterfaceManagementResponse::InterfaceManagementResponse),
            Err(e) => {
                warn!(logger, "Failed NG Setup send - {:?}", e);
                Ok(TriggerInterfaceManagementResponse::UnexpectedError(
                    models::Error {
                        code: 0,
                        message: format!("Failed {}: {:?}", interface_management_req.procedure, e),
                    },
                ))
            }
        }
    }
}
