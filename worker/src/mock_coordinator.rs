use crate::ClientContext;
use async_trait::async_trait;
use node_control_api::{models, Api, RefreshWorkerResponse};
use swagger::ApiError;

#[derive(Clone, Debug)]
pub struct MockCoordinator;

#[async_trait]
impl Api<ClientContext> for MockCoordinator {
    /// Refresh worker request
    async fn refresh_worker(
        &self,
        _refesh_worker_req: models::RefeshWorkerReq,
        context: &ClientContext,
    ) -> Result<RefreshWorkerResponse, ApiError> {
        let _context = context.clone();
        Err("Generic failure".into())
    }
}
