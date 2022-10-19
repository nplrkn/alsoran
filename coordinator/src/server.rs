use anyhow::Result;
use async_channel::Sender;
use async_trait::async_trait;
use common::ShutdownHandle;
use coordination_api::models::{self, WorkerInfo};
use coordination_api::server::MakeService;
use coordination_api::RefreshWorkerResponse;
use coordination_api::{context::MakeAddContext, Api};
use slog::{error, info, Logger};
use std::marker::PhantomData;
use std::net::SocketAddr;
use stop_token::StopSource;
use swagger::{ApiError, EmptyContext, Has, XSpanIdString};

use crate::Config;

#[derive(Clone)]
pub struct Server<C> {
    logger: Logger,
    marker: PhantomData<C>,
    sender: Sender<WorkerInfo>,
}

impl<C> Server<C> {
    pub fn new(sender: Sender<WorkerInfo>, logger: Logger) -> Self {
        Server {
            logger,
            marker: PhantomData,
            sender,
        }
    }
}

pub fn spawn(config: Config, logger: Logger) -> Result<ShutdownHandle> {
    info!(logger, "Coordinator instance start");
    let stop_source = StopSource::new();
    let stop_token = stop_source.token();

    let (sender, receiver) = async_channel::bounded(1);

    let addr = SocketAddr::new("127.0.0.1".parse()?, config.bind_port);
    let server = Server::new(sender, logger.clone());
    let service = MakeService::new(server);
    let service = MakeAddContext::<_, EmptyContext>::new(service);

    let server_task = async_std::task::spawn(async move {
        let control_task = super::control::spawn(receiver, stop_token.clone(), logger.clone());
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
        let context = context.clone();
        // info!(
        //     "refresh_worker({:?}) - X-Span-ID: {:?}",
        //     worker_info,
        //     context.get().0.clone()
        // );
        Err(ApiError("Generic failure".into()))
    }
}
