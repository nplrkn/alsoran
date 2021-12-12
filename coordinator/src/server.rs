//! Main library entry point for node_control_api implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::SslAcceptorBuilder;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use swagger::{Has, XSpanIdString};
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use node_control_api::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str) {
    let addr = addr.parse().expect("Failed to parse bind address");
    let server = Server::new();
    let service = MakeService::new(server);
    let service = MakeAllowAllAuthenticator::new(service, "cosmo");
    let service =
        node_control_api::server::context::MakeAddContext::<_, EmptyContext>::new(service);
    hyper::server::Server::bind(&addr)
        .serve(service)
        .await
        .unwrap()
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server {
            marker: PhantomData,
        }
    }
}

use node_control_api::server::MakeService;
use node_control_api::{Api, RefreshWorkerResponse};
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C>
where
    C: Has<XSpanIdString> + Send + Sync,
{
    /// Refresh worker request
    async fn refresh_worker(
        &self,
        refesh_worker_req: models::RefeshWorkerReq,
        context: &C,
    ) -> Result<RefreshWorkerResponse, ApiError> {
        let context = context.clone();
        info!(
            "refresh_worker({:?}) - X-Span-ID: {:?}",
            refesh_worker_req,
            context.get().0.clone()
        );
        Err("Generic failure".into())
    }
}
