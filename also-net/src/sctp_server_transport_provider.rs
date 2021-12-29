use crate::sctp_tnla_pool::SctpTnlaPool;
use crate::Codec;
use crate::TnlaEventHandler;
use crate::Wrapper;
use crate::{ServerTransportProvider, TransportProvider};
use anyhow::Result;
use async_std::sync::Arc;
use async_std::task;
use async_std::task::JoinHandle;
use async_trait::async_trait;
use futures::stream::StreamExt;
use futures::{future, pin_mut};
use slog::{info, o, trace, Logger};
use std::fmt::Debug;
use stop_token::StopToken;

#[derive(Debug, Clone)]
pub struct SctpServerTransportProvider<C, P>
where
    C: Codec<Pdu = P> + Clone + Send + Sync + 'static,
    P: Send + Sync + Clone + 'static + Debug,
{
    tnla_pool: SctpTnlaPool,
    ppid: u32,
    codec: C,
}

impl<C, P> SctpServerTransportProvider<C, P>
where
    C: Codec<Pdu = P> + Clone + Send + Sync + 'static,
    P: Send + Sync + Clone + 'static + Debug,
{
    pub fn new(ppid: u32, codec: C) -> SctpServerTransportProvider<C, P> {
        SctpServerTransportProvider {
            tnla_pool: SctpTnlaPool::new(),
            ppid,
            codec,
        }
    }
}

#[async_trait]
impl<C, P> TransportProvider for SctpServerTransportProvider<C, P>
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

const MAX_LISTEN_BACKLOG: i32 = 5;

#[async_trait]
impl<C, P> ServerTransportProvider for SctpServerTransportProvider<C, P>
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
