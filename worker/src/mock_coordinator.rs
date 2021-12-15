use crate::ClientContext;
use async_channel::{Receiver, Sender};
use async_trait::async_trait;
use models::RefreshWorkerReq;
use node_control_api::{models, Api, RefreshWorkerResponse};
use swagger::ApiError;

pub enum NodeControlRequest {
    RefreshWorkerReq(RefreshWorkerReq),
}

pub enum NodeControlResponse {
    RefreshWorkerResponse(RefreshWorkerResponse),
}

#[derive(Debug, Clone)]
pub struct MockCoordinator {
    sender: Sender<NodeControlRequest>,
    receiver: Receiver<NodeControlResponse>,
}

impl MockCoordinator {
    pub fn new() -> (
        MockCoordinator,
        Sender<NodeControlResponse>,
        Receiver<NodeControlRequest>,
    ) {
        let (sender, their_receiver) = async_channel::unbounded();
        let (their_sender, receiver) = async_channel::unbounded();

        (
            MockCoordinator { sender, receiver },
            their_sender,
            their_receiver,
        )
    }
}

#[async_trait]
impl Api<ClientContext> for MockCoordinator {
    /// Refresh worker request
    async fn refresh_worker(
        &self,
        refresh_worker_req: models::RefreshWorkerReq,
        context: &ClientContext,
    ) -> Result<RefreshWorkerResponse, ApiError> {
        let _context = context.clone();
        //TODO logger from context info!(logger, "MockTransportProvider send message {:?}", message);

        self.sender
            .send(NodeControlRequest::RefreshWorkerReq(refresh_worker_req))
            .await
            .unwrap();

        if let Ok(NodeControlResponse::RefreshWorkerResponse(response)) = self.receiver.recv().await
        {
            Ok(response)
        } else {
            // TODO log
            panic!("Bad response from test script")
        }
    }
}
