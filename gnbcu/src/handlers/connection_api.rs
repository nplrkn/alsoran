//! Main library entry point for connection_api implementation.

use anyhow::Result;
use async_trait::async_trait;
use common::ShutdownHandle;
use connection_api::models::ConnectionInfo;
use connection_api::models::OperationType;
use connection_api::server::MakeService;
use connection_api::AddConnectionResponse;
use connection_api::Api;
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
impl<C, G: Gnbcu> Api<C> for ConnectionApiHandler<C, G>
where
    C: Clone + Has<XSpanIdString> + Send + Sync,
{
    /// Instructs a worker to add a connection
    async fn add_connection(
        &self,
        connection_info: ConnectionInfo,
        _context: &C,
    ) -> Result<AddConnectionResponse, ApiError> {
        match connection_info.operation_type {
            OperationType::AddE1 => {
                Workflow::new(&self.gnbcu, &self.logger)
                    .gnb_cu_cp_configuration_update(&connection_info.ip_address)
                    .await
            }
            OperationType::AddF1 => {
                Workflow::new(&self.gnbcu, &self.logger)
                    .gnb_cu_configuration_update(&connection_info.ip_address)
                    .await
            }
            OperationType::SetupNg => {
                Workflow::new(&self.gnbcu, &self.logger)
                    .ng_setup(&connection_info.ip_address)
                    .await
            }
            OperationType::JoinNg => {
                Workflow::new(&self.gnbcu, &self.logger)
                    .ran_configuration_update(&connection_info.ip_address)
                    .await
            }
        }
        .map(|revision_number| AddConnectionResponse::Success(revision_number))
        .or_else(|e| {
            warn!(
                self.logger,
                "Error trying to add connection - {}",
                e.to_string()
            );
            Ok(AddConnectionResponse::Failure(e.to_string()))
        })
    }
}
