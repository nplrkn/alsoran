//! Main library entry point for node_control_api implementation.
use crate::gnbcu::Gnbcu;
use crate::{ClientContext, F1ServerTransportProvider, NgapClientTransportProvider};
use async_trait::async_trait;
use node_control_api::{models, Api, CallbackApi, TriggerInterfaceManagementResponse};
//use std::marker::PhantomData;
use swagger::ApiError;
//use swagger::{Has, XSpanIdString};

// #[derive(Copy, Clone)]
// pub struct NodeControlCallbackServer<C> {
//     marker: PhantomData<C>,
// }

// impl<C> NodeControlCallbackServer<C> {
//     pub fn new() -> Self {
//         NodeControlCallbackServer {
//             marker: PhantomData,
//         }
//     }
// }

#[async_trait]
impl<T, F, C, Cx> CallbackApi<Cx> for Gnbcu<T, F, C>
where
    T: NgapClientTransportProvider,
    F: F1ServerTransportProvider,
    C: Api<ClientContext> + Send + Sync + 'static + Clone,
    Cx: Send + Sync,
{
    /// A worker is instructed to trigger an interface management procedure on the given TNLA.
    async fn trigger_interface_management(
        &self,
        _tnla_id: i32,
        _interface_management_req: models::InterfaceManagementReq,
        _context: &Cx,
    ) -> Result<TriggerInterfaceManagementResponse, ApiError> {
        // let context = context.clone();
        // info!(
        //     "trigger_interface_management({}, {:?}) - X-Span-ID: {:?}",
        //     tnla_id,
        //     interface_management_req,
        //     context.get().0.clone()
        // );
        Err("Generic failure".into())
    }
}