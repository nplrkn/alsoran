use async_channel::Sender;
use async_trait::async_trait;
use node_control_api::models::{RefreshWorkerReq, RefreshWorkerRsp, TransportAddress};
use node_control_api::{Api, RefreshWorkerResponse};
use slog::{error, trace, Logger};
use std::marker::PhantomData;
use swagger::ApiError;
use swagger::{Has, XSpanIdString};

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
