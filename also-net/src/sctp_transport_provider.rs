use super::sctp_tnla_pool::SctpTnlaPool;
use crate::ServerTransportProvider;
use crate::{ClientTransportProvider, Codec, TnlaEventHandler, TransportProvider, Wrapper};
use anyhow::{anyhow, Result};
use async_std::sync::Arc;
use async_std::task;
use async_trait::async_trait;
use futures::future;
use futures::pin_mut;
use futures::stream::StreamExt;
use sctp::SctpAssociation;
use slog::{info, o, trace, warn, Logger};
use std::fmt::Debug;
use std::net::SocketAddr;
use std::time::Duration;
use stop_token::StopToken;
use task::JoinHandle;

#[derive(Debug, Clone)]
pub struct SctpTransportProvider<C, P>
where
    C: Codec<Pdu = P> + Clone + Send + Sync + 'static,
    P: Send + Sync + Clone + 'static + Debug,
{
    tnla_pool: SctpTnlaPool,
    ppid: u32,
    codec: C,
}

impl<C, P> SctpTransportProvider<C, P>
where
    C: Codec<Pdu = P> + Clone + Send + Sync + 'static,
    P: Send + Sync + Clone + 'static + Debug,
{
    pub fn new(ppid: u32, codec: C) -> SctpTransportProvider<C, P> {
        SctpTransportProvider {
            tnla_pool: SctpTnlaPool::new(),
            ppid,
            codec,
        }
    }
}

async fn resolve_and_connect(
    connect_addr_string: &str,
    ppid: u32,
    logger: &Logger,
) -> Result<SctpAssociation> {
    let addr = async_net::resolve(connect_addr_string)
        .await?
        .into_iter()
        .next()
        .ok_or(anyhow!("Address resolved to empty array"))?; // TODO - don't know if this is actually hittable
    SctpAssociation::establish(addr, ppid, logger).await
}

#[async_trait]
impl<C, P> TransportProvider for SctpTransportProvider<C, P>
where
    C: Codec<Pdu = P> + Clone + Send + Sync + 'static,
    P: Send + Sync + Clone + 'static + Debug,
{
    type Pdu = P;
    async fn send_pdu(&self, pdu: P, logger: &Logger) -> Result<()> {
        let message: Vec<u8> = self.codec.to_wire(pdu)?;
        self.tnla_pool.send_message(message, logger).await
    }
}

#[async_trait]
impl<C, P> ClientTransportProvider for SctpTransportProvider<C, P>
where
    C: Codec<Pdu = P> + Clone + Send + Sync + 'static,
    P: Send + Sync + Clone + 'static + Debug,
{
    type Pdu = P;
    async fn maintain_connection<H>(
        self,
        connect_addr_string: String,
        handler: H,
        stop_token: StopToken,
        logger: Logger,
    ) -> Result<JoinHandle<()>>
    where
        H: TnlaEventHandler<MessageType = <Self as ClientTransportProvider>::Pdu>,
    {
        let assoc_id = 3; // TODO
        let wrapped_handler = Wrapper {
            handler,
            codec: self.codec,
        };

        let task = task::spawn(async move {
            loop {
                match resolve_and_connect(&connect_addr_string, self.ppid, &logger).await {
                    Ok(assoc) => {
                        let logger = logger.new(o!("connection" => assoc_id));
                        info!(logger, "Established connection");

                        self.tnla_pool
                            .add_and_handle_no_spawn(
                                assoc_id,
                                Arc::new(assoc),
                                wrapped_handler.clone(),
                                stop_token.clone(),
                                logger.clone(),
                            )
                            .await;
                        warn!(logger, "SCTP connection terminated - will retry");
                    }
                    Err(e) => {
                        warn!(
                            logger,
                            "Couldn't establish connection - will retry ({:?})", e
                        );
                    }
                };
                let retry_duration = Duration::from_secs(30);
                if async_std::future::timeout(retry_duration, stop_token.clone())
                    .await
                    .is_ok()
                {
                    break;
                }
            }
        });
        Ok(task)
    }

    // Return the set of TNLA remote address to which we are currently connected
    async fn remote_tnla_addresses(&self) -> Vec<SocketAddr> {
        self.tnla_pool.remote_addresses().await
    }
}

const MAX_LISTEN_BACKLOG: i32 = 5;

#[async_trait]
impl<C, P> ServerTransportProvider for SctpTransportProvider<C, P>
where
    C: Codec<Pdu = P> + Clone + Send + Sync + 'static,
    P: Send + Sync + Clone + 'static + Debug,
{
    type Pdu = P;
    async fn serve<H>(
        self,
        listen_addr: String,
        stop_token: StopToken,
        handler: H,
        logger: Logger,
    ) -> Result<JoinHandle<()>>
    where
        H: TnlaEventHandler<MessageType = P>,
    {
        let addr = async_net::resolve(listen_addr)
            .await
            .map(|vec| vec[0])?
            .into();

        let wrapped_handler = Wrapper {
            handler,
            codec: self.codec,
        };

        Ok(task::spawn(async move {
            let stream = sctp::new_listen(addr, self.ppid, MAX_LISTEN_BACKLOG, logger.clone())
                .take_until(stop_token.clone());
            pin_mut!(stream);

            info!(logger, "Listening for connections");
            let mut connection_tasks = vec![];
            while let Some(Ok(assoc)) = stream.next().await {
                let assoc_id = 53; // TODO
                let logger = logger.new(o!("connection" => assoc_id));
                info!(logger, "Accepted connection");
                let task = self
                    .tnla_pool
                    .clone()
                    .add_and_handle(
                        assoc_id,
                        Arc::new(assoc),
                        wrapped_handler.clone(),
                        stop_token.clone(),
                        logger,
                    )
                    .await;
                connection_tasks.push(task);
            }

            info!(logger, "Graceful shutdown");
            trace!(logger, "Wait for connection tasks to finish");
            future::join_all(connection_tasks).await;
            trace!(logger, "Connection tasks finished");
        }))
    }
}
