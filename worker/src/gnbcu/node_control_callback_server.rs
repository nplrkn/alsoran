//! Main library entry point for node_control_api implementation.
use crate::gnbcu::Gnbcu;
use crate::ClientContext;
use anyhow::{anyhow, Result};
use async_std::task::JoinHandle;
use async_trait::async_trait;
use bitvec::prelude::*;
use net::{TransactionSender, TransportProvider};
use ngap::*;
use node_control_api::client::callbacks::MakeService;
use node_control_api::{models, Api, CallbackApi, TriggerInterfaceManagementResponse};
use slog::{error, info, trace, warn, Logger};
use stop_token::StopToken;
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::ApiError;
use swagger::EmptyContext;
use xxap_transaction::{RequestError, RequestProvider};

impl<
        N: TransportProvider,
        F: TransportProvider,
        C: Api<ClientContext> + Send + Sync + Clone + 'static,
    > Gnbcu<N, F, C>
{
    pub fn start_callback_server(&self, stop_token: StopToken) -> Result<JoinHandle<()>> {
        let addr = format!("0.0.0.0:{}", self.config.callback_server_bind_port).parse()?;
        let service = MakeService::new(self.clone());
        let service = MakeAllowAllAuthenticator::new(service, "cosmo");
        let service =
            node_control_api::server::context::MakeAddContext::<_, EmptyContext>::new(service);
        let logger = self.logger.clone();
        Ok(async_std::task::spawn(async move {
            let server = hyper::server::Server::bind(&addr)
                .serve(service)
                .with_graceful_shutdown(stop_token);
            if let Err(e) = server.await {
                error!(logger, "Server error: {}", e);
            } else {
                info!(logger, "Server graceful shutdown");
            }
        }))
    }
}

#[async_trait]
impl<N, F, C, Cx> CallbackApi<Cx> for Gnbcu<N, F, C>
where
    N: TransportProvider,
    F: TransportProvider,
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
        trace!(logger, "Trigger {}", interface_management_req.procedure);

        let response = self
            .trigger_procedure(interface_management_req.procedure.as_str(), logger)
            .await
            .or_else(|e| {
                warn!(logger, "Failed NG Setup send - {:?}", e);
                Ok(TriggerInterfaceManagementResponse::UnexpectedError(
                    models::Error {
                        code: 0,
                        message: format!("Failed {}: {:?}", interface_management_req.procedure, e),
                    },
                ))
            });
        trace!(
            logger,
            "Send {} trigger response",
            interface_management_req.procedure
        );
        response
    }
}

// TODO should the following be a separate module.  Unclear to have them in
// node control callback server.
impl<N, F, C> Gnbcu<N, F, C>
where
    N: TransportProvider,
    F: TransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
{
    async fn trigger_procedure(
        &self,
        procedure: &str,
        logger: &Logger,
    ) -> Result<TriggerInterfaceManagementResponse> {
        trace!(logger, "Send request to AMF and wait for response");
        let logger_clone = logger.clone();

        // TODO handle errors

        match procedure {
            "ngsetup" => {
                let response = <TransactionSender<N> as RequestProvider<
                    NgapPdu,
                    NgSetupRequestProcedure,
                >>::request(
                    &self.ngap_transport_provider, self.ng_setup(), logger
                )
                .await;
                match response {
                    Ok(x) => trace!(logger_clone, "NgSetupResponse {:?}", x),
                    Err(RequestError::UnsuccessfulResponse(x)) => {
                        trace!(logger_clone, "NgSetupFailure {:?}", x)
                    }
                    Err(RequestError::Other(s)) => trace!(logger_clone, "Other error {}", s),
                }
                Ok(())
            }

            "ranconfigurationupdate" => {
                let response = <TransactionSender<N> as RequestProvider<
                    NgapPdu,
                    RanConfigurationUpdate,
                >>::request(
                    &self.ngap_transport_provider, self.ng_setup(), logger
                )
                .await;
                match response {
                    Ok(x) => trace!(logger_clone, "NgSetupResponse {:?}", x),
                    Err(RequestError::UnsuccessfulResponse(x)) => {
                        trace!(logger_clone, "NgSetupFailure {:?}", x)
                    }
                    Err(RequestError::Other(s)) => trace!(logger_clone, "Other error {}", s),
                }
                Ok(())
            }
            x => Err(anyhow!(format!("Unknown procedure requested {}", x))),
        }?;
        trace!(logger, "Got response from AMF, respond to coordinator");
        Ok(TriggerInterfaceManagementResponse::InterfaceManagementResponse)
    }

    fn ng_setup(&self) -> NgSetupRequest {
        NgSetupRequest {
            global_ran_node_id: self.global_ran_node_id(),
            ran_node_name: None,
            supported_ta_list: SupportedTaList(vec![SupportedTaItem {
                tac: Tac(vec![0, 1, 2]),
                broadcast_plmn_list: BroadcastPlmnList(vec![BroadcastPlmnItem {
                    plmn_identity: PlmnIdentity(vec![2, 3, 2]),
                    tai_slice_support_list: SliceSupportList(vec![SliceSupportItem {
                        s_nssai: SNssai {
                            sst: Sst(vec![0x01]),
                            sd: None,
                        },
                    }]),
                }]),
            }]),
            default_paging_drx: PagingDrx::V128,
            ue_retention_information: None,
            nb_iot_default_paging_drx: None,
            extended_ran_node_name: None,
        }
    }

    fn ran_configuration_update(&self) -> NgapPdu {
        NgapPdu::InitiatingMessage(InitiatingMessage::RanConfigurationUpdate(
            RanConfigurationUpdate {
                ran_node_name: None,
                supported_ta_list: None,
                default_paging_drx: None,
                global_ran_node_id: Some(self.global_ran_node_id()),
                ngran_tnl_association_to_remove_list: None,
                nb_iot_default_paging_drx: None,
                extended_ran_node_name: None,
            },
        ))
    }

    fn global_ran_node_id(&self) -> GlobalRanNodeId {
        GlobalRanNodeId::GlobalGnbId(GlobalGnbId {
            plmn_identity: PlmnIdentity(vec![2, 3, 2]),
            gnb_id: GnbId::GnbId(bitvec![Msb0,u8; 1; 22]),
        })
    }
}
