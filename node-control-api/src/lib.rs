#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &'static str = "/v1";
pub const API_VERSION: &'static str = "1.0.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum RefreshWorkerResponse {
    /// Refresh worker response
    RefreshWorkerResponse
    (models::RefreshWorkerRsp)
    ,
    /// Unexpected error
    UnexpectedError
    (models::Error)
}

/// API
#[async_trait]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Refresh worker request
    async fn refresh_worker(
        &self,
        refresh_worker_req: models::RefreshWorkerReq,
        context: &C) -> Result<RefreshWorkerResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Refresh worker request
    async fn refresh_worker(
        &self,
        refresh_worker_req: models::RefreshWorkerReq,
        ) -> Result<RefreshWorkerResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self: Self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Refresh worker request
    async fn refresh_worker(
        &self,
        refresh_worker_req: models::RefreshWorkerReq,
        ) -> Result<RefreshWorkerResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().refresh_worker(refresh_worker_req, &context).await
    }

}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum TriggerInterfaceManagementResponse {
    /// Interface management response
    InterfaceManagementResponse
    ,
    /// Unexpected error
    UnexpectedError
    (models::Error)
}


/// Callback API
#[async_trait]
pub trait CallbackApi<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// A worker is instructed to trigger an interface management procedure on the given TNLA.
    async fn trigger_interface_management(
        &self,
        callback_request_body_callback_url: String,
        interface_management_req: models::InterfaceManagementReq,
        context: &C) -> Result<TriggerInterfaceManagementResponse, ApiError>;

}

/// Callback API without a `Context`
#[async_trait]
pub trait CallbackApiNoContext<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// A worker is instructed to trigger an interface management procedure on the given TNLA.
    async fn trigger_interface_management(
        &self,
        callback_request_body_callback_url: String,
        interface_management_req: models::InterfaceManagementReq,
        ) -> Result<TriggerInterfaceManagementResponse, ApiError>;

}

pub trait CallbackContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self: Self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: CallbackApi<C> + Send + Sync, C: Clone + Send + Sync> CallbackContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: CallbackApi<C> + Send + Sync, C: Clone + Send + Sync> CallbackApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// A worker is instructed to trigger an interface management procedure on the given TNLA.
    async fn trigger_interface_management(
        &self,
        callback_request_body_callback_url: String,
        interface_management_req: models::InterfaceManagementReq,
        ) -> Result<TriggerInterfaceManagementResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().trigger_interface_management(
            callback_request_body_callback_url,
            interface_management_req,
            &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(any(feature = "client", feature = "server"))]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
