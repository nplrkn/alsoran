//! Main library entry point for node_control_api implementation.
use crate::gnbcu::Gnbcu;
use crate::{ClientContext, F1ServerTransportProvider, NgapClientTransportProvider};
use anyhow::{anyhow, Result};
use async_std::task::JoinHandle;
use async_trait::async_trait;
use bitvec::vec::BitVec;
use ngap::*;
use node_control_api::client::callbacks::MakeService;
use node_control_api::{models, Api, CallbackApi, TriggerInterfaceManagementResponse};
use slog::{error, info, trace, warn, Logger};
use stop_token::StopToken;
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::ApiError;
use swagger::EmptyContext;

impl<
        T: NgapClientTransportProvider,
        F: F1ServerTransportProvider,
        C: Api<ClientContext> + Send + Sync + Clone + 'static,
    > Gnbcu<T, F, C>
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
impl<T, F, C> Gnbcu<T, F, C>
where
    T: NgapClientTransportProvider,
    F: F1ServerTransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
{
    async fn trigger_procedure(
        &self,
        procedure: &str,
        logger: &Logger,
    ) -> Result<TriggerInterfaceManagementResponse> {
        trace!(logger, "Send request to AMF and wait for response");
        let logger_clone = logger.clone();
        let _response = match procedure {
            "ngsetup" => {
                let match_fn = Box::new(move |p: &NgapPdu| match p {
                    NgapPdu::SuccessfulOutcome(SuccessfulOutcome::NgSetupResponse(x)) => {
                        trace!(logger_clone, "NgSetupResponse {:?}", x);
                        true
                    }
                    NgapPdu::UnsuccessfulOutcome(UnsuccessfulOutcome::NgSetupFailure(x)) => {
                        trace!(logger_clone, "NgSetupFailure {:?}", x);
                        true
                    }
                    _ => false,
                });

                self.ngap_transport_provider
                    .send_request(self.ng_setup(), match_fn, logger)
                    .await
            }

            "ranconfigurationupdate" => {
                let match_fn = Box::new(move |p: &NgapPdu| match p {
                    NgapPdu::SuccessfulOutcome(
                        SuccessfulOutcome::RanConfigurationUpdateAcknowledge(x),
                    ) => {
                        trace!(logger_clone, "RanConfigurationUpdateAcknowledge {:?}", x);
                        true
                    }
                    NgapPdu::UnsuccessfulOutcome(
                        UnsuccessfulOutcome::RanConfigurationUpdateFailure(x),
                    ) => {
                        trace!(logger_clone, "RanConfigurationUpdateFailure {:?}", x);
                        true
                    }
                    _ => false,
                });
                self.ngap_transport_provider
                    .send_request(self.ran_configuration_update(), match_fn, logger)
                    .await
            }
            x => Err(anyhow!(format!("Unknown procedure requested {}", x))),
        }?;
        trace!(logger, "Got response from AMF, respond to coordinator");
        Ok(TriggerInterfaceManagementResponse::InterfaceManagementResponse)
    }

    fn ng_setup(&self) -> NgapPdu {
        NgapPdu::InitiatingMessage(InitiatingMessage::NgSetupRequest(NgSetupRequest {
            global_ran_node_id: self.global_ran_node_id(),
            ran_node_name: None,
            supported_ta_list: SupportedTaList(vec![]),
            default_paging_drx: PagingDrx::V128,
            ue_retention_information: None,
            nb_iot_default_paging_drx: None,
            extended_ran_node_name: None,
        }))
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
            plmn_identity: PlmnIdentity(vec![2, 3, 2, 1, 5, 6]),
            gnb_id: GnbId::GnbId(BitVec::from_element(0x10)),
        })
    }
}
