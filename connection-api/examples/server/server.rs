//! Main library entry point for connection_api implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use connection_api::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    #[allow(unused_mut)]
    let mut service =
        connection_api::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set certificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let tcp_listener = TcpListener::bind(&addr).await.unwrap();

            loop {
                if let Ok((tcp, _)) = tcp_listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        Http::new()
                            .serve_connection(tls, service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use connection_api::{
    Api,
    AddE1apResponse,
    AddF1apResponse,
    JoinNgapResponse,
    SetupNgapResponse,
};
use connection_api::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// Instructs a worker to add another worker to an existing E1AP interface instance
    async fn add_e1ap(
        &self,
        transport_address: models::TransportAddress,
        context: &C) -> Result<AddE1apResponse, ApiError>
    {
        let context = context.clone();
        info!("add_e1ap({:?}) - X-Span-ID: {:?}", transport_address, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Instructs a worker to add another worker to an existing F1AP interface instance
    async fn add_f1ap(
        &self,
        transport_address: models::TransportAddress,
        context: &C) -> Result<AddF1apResponse, ApiError>
    {
        let context = context.clone();
        info!("add_f1ap({:?}) - X-Span-ID: {:?}", transport_address, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Instructs a worker to join an existing NGAP interface instance set up by another worker.
    async fn join_ngap(
        &self,
        transport_address: models::TransportAddress,
        context: &C) -> Result<JoinNgapResponse, ApiError>
    {
        let context = context.clone();
        info!("join_ngap({:?}) - X-Span-ID: {:?}", transport_address, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Instructs a worker to set up an NGAP interface instance with the AMF
    async fn setup_ngap(
        &self,
        transport_address: models::TransportAddress,
        context: &C) -> Result<SetupNgapResponse, ApiError>
    {
        let context = context.clone();
        info!("setup_ngap({:?}) - X-Span-ID: {:?}", transport_address, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

}
