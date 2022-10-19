//! Main library entry point for connection_api implementation.

use anyhow::Result;
use async_trait::async_trait;
use common::ShutdownHandle;
use connection_api::models;
use connection_api::server::MakeService;
use connection_api::{AddE1apResponse, AddF1apResponse, Api, JoinNgapResponse, SetupNgapResponse};
use slog::{error, info, Logger};
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
            info!(logger, "Server graceful shutdown");
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
        _transport_address: models::TransportAddress,
        _context: &C,
    ) -> Result<AddE1apResponse, ApiError> {
        Workflow::new(&self.gnbcu, &self.logger)
            .add_e1ap_endpoint()
            .await;
        Err(ApiError("Generic failure".into()))
    }

    /// Instructs a worker to add another worker to an existing F1AP interface instance
    async fn add_f1ap(
        &self,
        _transport_address: models::TransportAddress,
        _context: &C,
    ) -> Result<AddF1apResponse, ApiError> {
        //Workflow::new(&self.gnbcu, logger).???.await
        Err(ApiError("Generic failure".into()))
    }

    /// Instructs a worker to join an existing NGAP interface instance set up by another worker.
    async fn join_ngap(
        &self,
        _transport_address: models::TransportAddress,
        _context: &C,
    ) -> Result<JoinNgapResponse, ApiError> {
        //Workflow::new(&self.gnbcu, logger).???.await
        Err(ApiError("Generic failure".into()))
    }

    /// Instructs a worker to set up an NGAP interface instance with the AMF
    async fn setup_ngap(
        &self,
        _transport_address: models::TransportAddress,
        _context: &C,
    ) -> Result<SetupNgapResponse, ApiError> {
        //Workflow::new(&self.gnbcu, logger).???.await
        Err(ApiError("Generic failure".into()))
    }
}
