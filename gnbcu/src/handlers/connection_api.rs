//! Main library entry point for connection_api implementation.

use anyhow::Result;
use async_trait::async_trait;
use common::ShutdownHandle;
use connection_api::models::{AmfInfo, IpAddress};
use connection_api::server::MakeService;
use connection_api::{AddE1apResponse, AddF1apResponse, Api, JoinNgapResponse, SetupNgapResponse};
use ngap::AmfName;
use slog::{debug, error, warn, Logger};
use std::marker::PhantomData;
use std::net::SocketAddr;
use stop_token::StopSource;
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::ApiError;
use swagger::EmptyContext;
use swagger::{Has, XSpanIdString};

use crate::gnbcu_trait::Gnbcu;
use crate::workflows::Workflow;

#[derive(Clone)]
pub struct ConnectionApiHandler<C, G: Gnbcu> {
    gnbcu: G,
    logger: Logger,
    marker: PhantomData<C>,
}

impl<C, G: Gnbcu> ConnectionApiHandler<C, G> {
    pub fn new(gnbcu: G, logger: Logger) -> Self {
        ConnectionApiHandler {
            gnbcu,
            logger,
            marker: PhantomData,
        }
    }
}

pub async fn serve<G: Gnbcu>(addr: SocketAddr, gnbcu: G, logger: Logger) -> Result<ShutdownHandle> {
    let stop_source = StopSource::new();
    let stop_token = stop_source.token();
    // A key trick in this function is that the awkwardly complex type parameter C is inferred at the point of the call to
    // ConnectionApiHandler::new().  For this reason it does not work to simply make this function a method of
    // ConnectionApiHandler - in that case, the type parameter would need to be made explicit on the new() call.
    let server = ConnectionApiHandler::new(gnbcu, logger.clone());
    let service = MakeService::new(server);
    let service = MakeAllowAllAuthenticator::new(service, "cosmo");
    let service = connection_api::server::context::MakeAddContext::<_, EmptyContext>::new(service);
    let server_task = async_std::task::spawn(async move {
        let server = hyper::server::Server::bind(&addr)
            .serve(service)
            .with_graceful_shutdown(stop_token);
        if let Err(e) = server.await {
            error!(logger, "Server error: {}", e);
        } else {
            debug!(logger, "Connection API server graceful shutdown");
        }
    });

    Ok(ShutdownHandle::new(server_task, stop_source))
}

#[async_trait]
impl<C: Clone, G: Gnbcu> Api<C> for ConnectionApiHandler<C, G>
where
    C: Has<XSpanIdString> + Send + Sync,
{
    /// Instructs a worker to add another worker to an existing E1AP interface instance
    async fn add_e1ap(
        &self,
        transport_address: IpAddress,
        _context: &C,
    ) -> Result<AddE1apResponse, ApiError> {
        match Workflow::new(&self.gnbcu, &self.logger)
            .add_e1ap_endpoint(&transport_address)
            .await
        {
            Ok(_) => Ok(AddE1apResponse::Success),
            Err(e) => {
                warn!(self.logger, "E1AP add failed - {:?}", e);
                Ok(AddE1apResponse::Failure(format!(
                    "E1AP add of {} failed",
                    transport_address.to_string()
                )))
            }
        }
    }

    /// Instructs a worker to add another worker to an existing F1AP interface instance
    async fn add_f1ap(
        &self,
        transport_address: IpAddress,
        _context: &C,
    ) -> Result<AddF1apResponse, ApiError> {
        match Workflow::new(&self.gnbcu, &self.logger)
            .add_f1ap_endpoint(&transport_address)
            .await
        {
            Ok(_) => Ok(AddF1apResponse::Success),
            Err(e) => {
                warn!(self.logger, "F1AP add failed - {:?}", e);
                Ok(AddF1apResponse::Failure(format!(
                    "F1AP add of {} failed",
                    transport_address.to_string()
                )))
            }
        }
    }

    /// Instructs a worker to join an existing NGAP interface instance set up by another worker.
    async fn join_ngap(
        &self,
        transport_address: IpAddress,
        _context: &C,
    ) -> Result<JoinNgapResponse, ApiError> {
        // First establish a connection.
        if let Err(e) = self.gnbcu.ngap_connect(&transport_address).await {
            error!(self.logger, "Failed to connect- {}", e);
            return Ok(JoinNgapResponse::Failure(format!(
                "Failed to connect to AMF at {}",
                transport_address.to_string()
            )));
        }

        // Carry out Configuration Update.
        match Workflow::new(&self.gnbcu, &self.logger)
            .ran_configuration_update()
            .await
        {
            Ok(()) => Ok(JoinNgapResponse::Success),
            Err(e) => {
                warn!(self.logger, "NG join failed - {:?}", e);
                Ok(JoinNgapResponse::Failure(
                    "Failed RAN configuration update with AMF".to_string(),
                ))
            }
        }
    }

    /// Instructs a worker to set up an NGAP interface instance with the AMF
    async fn setup_ngap(
        &self,
        transport_address: IpAddress,
        _context: &C,
    ) -> Result<SetupNgapResponse, ApiError> {
        // First establish a connection.
        if let Err(e) = self.gnbcu.ngap_connect(&transport_address).await {
            error!(self.logger, "Failed to connect - {}", e);
            return Ok(SetupNgapResponse::Failure(format!(
                "Failed to connect to AMF at {}",
                transport_address.to_string()
            )));
        }

        // Then carry out NG Setup
        match Workflow::new(&self.gnbcu, &self.logger).ng_setup().await {
            Ok(AmfName(amf_name)) => Ok(SetupNgapResponse::Success(AmfInfo { amf_name })),

            Err(e) => {
                warn!(self.logger, "NG Setup failed - {:?}", e);
                Ok(SetupNgapResponse::Failure(
                    "Failed NG Setup to AMF".to_string(),
                ))
            }
        }
    }
}
