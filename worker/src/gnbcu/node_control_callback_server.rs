//! Main library entry point for node_control_api implementation.
use crate::gnbcu::Gnbcu;
use crate::{ClientContext, F1ServerTransportProvider, NgapClientTransportProvider};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bitvec::vec::BitVec;
use common::ngap::*;
use node_control_api::{models, Api, CallbackApi, TriggerInterfaceManagementResponse};
use slog::{info, warn, Logger};
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
        _callback_url: String,
        interface_management_req: models::InterfaceManagementReq,
        _context: &Cx,
    ) -> Result<TriggerInterfaceManagementResponse, ApiError> {
        let logger = &self.logger;

        // TODO the interface management request ought to specify which TNLA ID to send to
        // and we ought to specify that in the send_pdu call.
        //
        // The idea of using a TNLA UUID rather than AMF address comes from the fact that
        // a connection can toggle.  The TNLA ID is the connection instance from a series
        // of connections to the same AMF endpoint.  If we cannot target a connection instance
        // then we cannot ensure that each connection has been correctly initialized.
        info!(logger, "Trigger {}", interface_management_req.procedure);

        match self
            .trigger_procedure(interface_management_req.procedure.as_str(), logger)
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

impl<T, F, C> Gnbcu<T, F, C>
where
    T: NgapClientTransportProvider,
    F: F1ServerTransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
{
    async fn trigger_procedure(&self, procedure: &str, logger: &Logger) -> Result<()> {
        let pdu = match procedure {
            "ngsetup" => Ok(self.ng_setup()),
            "ranconfigurationupdate" => Ok(self.ran_configuration_update()),
            x => Err(anyhow!(format!("Unknown procedure requested {}", x))),
        }?;

        // TODO use tnla_id
        self.ngap_transport_provider.send_pdu(pdu, logger).await
    }

    fn ng_setup(&self) -> NgapPdu {
        NgapPdu::InitiatingMessage(InitiatingMessage {
            procedure_code: ProcedureCode(21),
            criticality: Criticality(Criticality::REJECT),
            value: InitiatingMessageValue::IdNgSetup(NgSetupRequest {
                protocol_i_es: NgSetupRequestProtocolIEs(vec![NgSetupRequestProtocolIEsItem {
                    id: ProtocolIeId(27),
                    criticality: Criticality(Criticality::REJECT),
                    value: NgSetupRequestProtocolIEsItemValue::IdGlobalRanNodeId(
                        self.global_ran_node_id(),
                    ),
                }]),
            }),
        })
    }

    fn ran_configuration_update(&self) -> NgapPdu {
        NgapPdu::InitiatingMessage(InitiatingMessage {
            procedure_code: ProcedureCode(35),
            criticality: Criticality(Criticality::REJECT),
            value: InitiatingMessageValue::IdRanConfigurationUpdate(RanConfigurationUpdate {
                protocol_i_es: RanConfigurationUpdateProtocolIEs(vec![
                    RanConfigurationUpdateProtocolIEsItem {
                        id: ProtocolIeId(27),
                        criticality: Criticality(Criticality::REJECT),
                        value: RanConfigurationUpdateProtocolIEsItemValue::IdGlobalRanNodeId(
                            self.global_ran_node_id(),
                        ),
                    },
                ]),
            }),
        })
    }

    fn global_ran_node_id(&self) -> GlobalRanNodeId {
        GlobalRanNodeId::GlobalGnbId(GlobalGnbId {
            plmn_identity: PlmnIdentity(vec![2, 3, 2, 1, 5, 6]),
            gnb_id: GnbId::GnbId(BitString26(BitVec::from_element(0x10))),
            ie_extensions: None,
        })
    }
}
