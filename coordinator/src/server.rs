use anyhow::Result;
use async_channel::Sender;
use async_trait::async_trait;
use common::ShutdownHandle;
use node_control_api::models::{RefreshWorkerReq, RefreshWorkerRsp, TransportAddress};
use node_control_api::server::MakeService;
use node_control_api::{Api, RefreshWorkerResponse};
use slog::{error, info, trace, Logger};
use std::marker::PhantomData;
use std::net::SocketAddr;
use stop_token::StopSource;
use swagger::ApiError;
use swagger::EmptyContext;
use swagger::{Has, XSpanIdString};

use crate::Config;

#[derive(Clone)]
pub struct Server<C> {
    logger: Logger,
    marker: PhantomData<C>,
    sender: Sender<RefreshWorkerReq>,
}

impl<C> Server<C> {
    pub fn new(sender: Sender<RefreshWorkerReq>, logger: Logger) -> Self {
        Server {
            logger,
            marker: PhantomData,
            sender,
        }
    }
}

impl<C> Server<C> {
    pub fn spawn(config: Config, logger: Logger) -> Result<ShutdownHandle> {
        info!(logger, "Coordinator instance start");
        let stop_source = StopSource::new();
        let stop_token = stop_source.token();

        let (sender, receiver) = async_channel::bounded(1);

        let addr = SocketAddr::new("127.0.0.1".parse()?, config.bind_port);
        let server = Server::new(sender, logger.clone());
        let service = MakeService::new(server);
        let service =
            node_control_api::server::context::MakeAddContext::<_, EmptyContext>::new(service);

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
}

#[async_trait]
impl<C> Api<C> for Server<C>
where
    C: Has<XSpanIdString> + Send + Sync,
{
    /// Refresh worker request
    async fn refresh_worker(
        &self,
        refresh_worker_req: RefreshWorkerReq,
        _context: &C,
    ) -> Result<RefreshWorkerResponse, ApiError> {
        //let context = context.clone();
        trace!(
            self.logger,
            "Refresh worker from {}",
            refresh_worker_req.worker_unique_id
        );

        // Signal the control task
        self.sender
            .send(refresh_worker_req)
            .await
            .unwrap_or_else(|_| {
                error!(self.logger, "Internal control channel unexpectedly closed")
            });

        Ok(RefreshWorkerResponse::RefreshWorkerResponse(
            RefreshWorkerRsp {
                amf_addresses: vec![TransportAddress {
                    host: "127.0.0.1".to_string(),
                    port: None,
                }],
            },
        ))
    }
}
