use async_channel::Sender;
use async_trait::async_trait;
use log::trace;
use node_control_api::models::{RefreshWorkerReq, RefreshWorkerRsp, TransportAddress};
use node_control_api::{Api, RefreshWorkerResponse};
use std::marker::PhantomData;
use swagger::ApiError;
use swagger::{Has, XSpanIdString};

#[derive(Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
    sender: Sender<RefreshWorkerReq>,
}

impl<C> Server<C> {
    pub fn new(sender: Sender<RefreshWorkerReq>) -> Self {
        Server {
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
        context: &C,
    ) -> Result<RefreshWorkerResponse, ApiError> {
        //let context = context.clone();
        trace!(
            "refresh_worker({:?}) - X-Span-ID: {:?}",
            refresh_worker_req,
            context.get().0.clone()
        );

        // Signal the control task

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
