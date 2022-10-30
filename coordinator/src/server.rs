use anyhow::Result;
use async_channel::{Receiver, Sender};
use async_trait::async_trait;
use common::ShutdownHandle;
use connection_api::{Api as ConnectionApi, Client};
use coordination_api::models::{self, WorkerInfo};
use coordination_api::server::MakeService;
use coordination_api::RefreshWorkerResponse;
use coordination_api::{context::MakeAddContext, Api};
use hyper::Body;
use slog::{error, info, Logger};
use std::marker::PhantomData;
use std::net::SocketAddr;
use stop_token::StopSource;
use swagger::{ApiError, DropContextService, EmptyContext, Has, XSpanIdString};

use crate::control::ClientContext;
use crate::{Config, ConnectionControlConfig};

#[derive(Clone)]
pub struct Server<C> {
    logger: Logger,
    marker: PhantomData<C>,
    sender: Sender<WorkerInfo>,
}

// To run local copy, we do new() then start().
// To run standalone server... - spawn().

impl<C> Server<C> {
    pub fn new(logger: Logger) -> (Self, Receiver<WorkerInfo>) {
        let (sender, receiver) = async_channel::bounded(1);
        (
            Server {
                logger,
                marker: PhantomData,
                sender,
            },
            receiver,
        )
    }

    // Start the control task
    pub fn start_with_local_api_provider<
        T: ConnectionApi<ClientContext> + Clone + Send + Sync + 'static,
    >(
        &self,
        connection_control_config: ConnectionControlConfig,
        receiver: Receiver<WorkerInfo>,
        local_api_provider: T,
    ) -> ShutdownHandle {
        let stop_source = StopSource::new();
        let stop_token = stop_source.token();

        let control_task = super::control::spawn(
            receiver,
            connection_control_config,
            stop_token,
            Some(local_api_provider),
            self.logger.clone(),
        );
        ShutdownHandle::new(control_task, stop_source)
    }

    pub fn start(
        &self,
        connection_control_config: ConnectionControlConfig,
        receiver: Receiver<WorkerInfo>,
    ) -> ShutdownHandle {
        let stop_source = StopSource::new();
        let stop_token = stop_source.token();

        // For the type parameter, we need any concrete type that implements the Api trait.
        // An HTTP client will do.
        let control_task = super::control::spawn::<
            Client<
                DropContextService<
                    hyper::client::Client<hyper::client::HttpConnector, Body>,
                    ClientContext,
                >,
                ClientContext,
            >,
        >(
            receiver,
            connection_control_config,
            stop_token,
            None,
            self.logger.clone(),
        );
        ShutdownHandle::new(control_task, stop_source)
    }
}

pub fn spawn(config: Config, logger: Logger) -> Result<ShutdownHandle> {
    info!(logger, "Coordinator instance start");
    let stop_source = StopSource::new();
    let stop_token = stop_source.token();

    let addr = SocketAddr::new("127.0.0.1".parse()?, config.bind_port);
    let (server, receiver) = Server::new(logger.clone());
    let control_task = server.start(config.connection_control_config, receiver);
    let service = MakeService::new(server);
    let service = MakeAddContext::<_, EmptyContext>::new(service);

    let server_task = async_std::task::spawn(async move {
        let server = hyper::server::Server::bind(&addr)
            .serve(service)
            .with_graceful_shutdown(stop_token);
        if let Err(e) = server.await {
            error!(logger, "Server error: {}", e);
        } else {
            info!(logger, "Server graceful shutdown");
        }
        control_task.graceful_shutdown().await;
    });
    Ok(ShutdownHandle::new(server_task, stop_source))
}

#[async_trait]
impl<C> Api<C> for Server<C>
where
    C: Has<XSpanIdString> + Send + Sync,
{
    /// Updates coordinator with information about a worker instance
    async fn refresh_worker(
        &self,
        worker_info: models::WorkerInfo,
        context: &C,
    ) -> Result<RefreshWorkerResponse, ApiError> {
        //let _context = context.clone();
        info!(
            self.logger,
            "refresh_worker({:?}) - X-Span-ID: {:?}",
            worker_info,
            context.get().0.clone()
        );
        // Signal the control task
        self.sender.send(worker_info).await.unwrap_or_else(|_| {
            error!(self.logger, "Internal control channel unexpectedly closed")
        });

        Err(ApiError("Generic failure".into()))
    }
}
