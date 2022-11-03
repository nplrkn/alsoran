#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::blacklisted_name)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "/v1";
pub const API_VERSION: &str = "1.0.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum AddE1apResponse {
    /// Cu Up accepted worker addition
    CuUpAcceptedWorkerAddition
    ,
    /// No connection
    NoConnection
    ,
    /// Failed add
    FailedAdd
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum AddF1apResponse {
    /// Du accepted worker addition
    DuAcceptedWorkerAddition
    ,
    /// No connection
    NoConnection
    ,
    /// Failed to add worker
    FailedToAddWorker
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum JoinNgapResponse {
    /// Success
    Success
    ,
    /// Failure
    Failure
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum SetupNgapResponse {
    /// Success
    Success
    (models::AmfInfo)
    ,
    /// Failure
    Failure
    (String)
}

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Instructs a worker to add another worker to an existing E1AP interface instance
    async fn add_e1ap(
        &self,
        transport_address: models::TransportAddress,
        context: &C) -> Result<AddE1apResponse, ApiError>;

    /// Instructs a worker to add another worker to an existing F1AP interface instance
    async fn add_f1ap(
        &self,
        transport_address: models::TransportAddress,
        context: &C) -> Result<AddF1apResponse, ApiError>;

    /// Instructs a worker to join an existing NGAP interface instance set up by another worker.
    async fn join_ngap(
        &self,
        transport_address: models::TransportAddress,
        context: &C) -> Result<JoinNgapResponse, ApiError>;

    /// Instructs a worker to set up an NGAP interface instance with the AMF
    async fn setup_ngap(
        &self,
        transport_address: models::TransportAddress,
        context: &C) -> Result<SetupNgapResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Instructs a worker to add another worker to an existing E1AP interface instance
    async fn add_e1ap(
        &self,
        transport_address: models::TransportAddress,
        ) -> Result<AddE1apResponse, ApiError>;

    /// Instructs a worker to add another worker to an existing F1AP interface instance
    async fn add_f1ap(
        &self,
        transport_address: models::TransportAddress,
        ) -> Result<AddF1apResponse, ApiError>;

    /// Instructs a worker to join an existing NGAP interface instance set up by another worker.
    async fn join_ngap(
        &self,
        transport_address: models::TransportAddress,
        ) -> Result<JoinNgapResponse, ApiError>;

    /// Instructs a worker to set up an NGAP interface instance with the AMF
    async fn setup_ngap(
        &self,
        transport_address: models::TransportAddress,
        ) -> Result<SetupNgapResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
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

    /// Instructs a worker to add another worker to an existing E1AP interface instance
    async fn add_e1ap(
        &self,
        transport_address: models::TransportAddress,
        ) -> Result<AddE1apResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().add_e1ap(transport_address, &context).await
    }

    /// Instructs a worker to add another worker to an existing F1AP interface instance
    async fn add_f1ap(
        &self,
        transport_address: models::TransportAddress,
        ) -> Result<AddF1apResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().add_f1ap(transport_address, &context).await
    }

    /// Instructs a worker to join an existing NGAP interface instance set up by another worker.
    async fn join_ngap(
        &self,
        transport_address: models::TransportAddress,
        ) -> Result<JoinNgapResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().join_ngap(transport_address, &context).await
    }

    /// Instructs a worker to set up an NGAP interface instance with the AMF
    async fn setup_ngap(
        &self,
        transport_address: models::TransportAddress,
        ) -> Result<SetupNgapResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().setup_ngap(transport_address, &context).await
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

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
