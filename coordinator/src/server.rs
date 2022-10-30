use anyhow::Result;
use async_channel::Sender;
use async_std::task::JoinHandle;
use async_trait::async_trait;
use common::ShutdownHandle;
use coordination_api::models::{self, WorkerInfo};
use coordination_api::server::MakeService;
use coordination_api::RefreshWorkerResponse;
use coordination_api::{context::MakeAddContext, Api};
use slog::{error, info, Logger};
use std::marker::PhantomData;
use std::net::SocketAddr;
use stop_token::{StopSource, StopToken};
use swagger::{ApiError, EmptyContext, Has, XSpanIdString};

use crate::Config;

#[derive(Clone)]
pub struct Server<C> {
    logger: Logger,
    marker: PhantomData<C>,
    sender: Sender<WorkerInfo>,
}

impl<C> Server<C> {
    pub fn new(stop_token: StopToken, config: Config, logger: Logger) -> (Self, JoinHandle<()>) {
        let (sender, receiver) = async_channel::bounded(1);
        let control_task = super::control::spawn(receiver, config, stop_token, logger.clone());

        (
            Server {
                logger,
                marker: PhantomData,
                sender,
            },
            control_task,
        )
    }
}

pub fn _spawn(config: Config, logger: Logger) -> Result<ShutdownHandle> {
    info!(logger, "Coordinator instance start");
    let stop_source = StopSource::new();
    let stop_token = stop_source.token();

    let addr = SocketAddr::new("127.0.0.1".parse()?, config.bind_port);
    let (server, control_task) = Server::new(stop_token.clone(), config, logger.clone());
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
        control_task.await;
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
