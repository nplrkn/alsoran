use super::sctp_tnla_pool::SctpTnlaPool;
use crate::{ClientTransportProvider, Codec, TnlaEventHandler, TransportProvider, Wrapper};
use anyhow::{anyhow, Result};
use async_std::future;
use async_std::sync::Arc;
use async_std::task;
use async_trait::async_trait;
use sctp::SctpAssociation;
use slog::{info, o, warn, Logger};
use std::fmt::Debug;
use std::time::Duration;
use stop_token::StopToken;
use task::JoinHandle;

#[derive(Debug, Clone)]
pub struct SctpClientTransportProvider<C, P>
where
    C: Codec<Pdu = P> + Clone + Send + Sync + 'static,
    P: Send + Sync + Clone + 'static + Debug,
{
    tnla_pool: SctpTnlaPool,
    ppid: u32,
    codec: C,
}

impl<C, P> SctpClientTransportProvider<C, P>
where
    C: Codec<Pdu = P> + Clone + Send + Sync + 'static,
    P: Send + Sync + Clone + 'static + Debug,
{
    pub fn new(ppid: u32, codec: C) -> SctpClientTransportProvider<C, P> {
        SctpClientTransportProvider {
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
        .ok_or(anyhow!("Address resolved to empty array"))? // Don't know if this is actually hittable
        .into();
    SctpAssociation::establish(addr, ppid, logger).await
}

#[async_trait]
impl<C, P> TransportProvider for SctpClientTransportProvider<C, P>
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
impl<C, P> ClientTransportProvider for SctpClientTransportProvider<C, P>
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
                if future::timeout(retry_duration, stop_token.clone())
                    .await
                    .is_ok()
                {
                    break;
                }
            }
        });
        Ok(task)
    }
}
