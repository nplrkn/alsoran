use super::f1ap_procedures::{CuInitiatedOperations, DuInitiatedOperations};
/// F1 transport used by the gNB-CU
use async_net::AsyncToSocketAddrs;
use async_trait::async_trait;

pub struct F1TransportCu {}

impl F1TransportCu {
    /// Create a new F1 transport for use by a gNB-CU.
    ///
    /// # Arguments
    ///
    /// * `listen_addr` - Transport address to listen on for F1 transport connections.
    /// * `handler` - Handler for requests and notifications sent by the gNB-DU.  
    pub async fn new<A: AsyncToSocketAddrs>(listen_addr: A) -> Result<F1TransportCu> {
        unimplemented!();
    }

    pub async fn set_handler<T: DuInitiatedOperations>(&mut self, &handler: T) {}
}

struct UeContextSetupResponse;
#[async_trait]
impl CuInitiatedOperations for F1TransportCu {
    async fn ue_context_setup_request() -> Result<UeContextSetupResponse, _> {
        unimplemented!();
    }
}
