use super::sctp_tnla_pool::SctpTnlaPool;
use crate::ngap::NgapPdu;
use crate::sctp::SctpAssociation;
use crate::tnla_event_handler::{JsonDecoder, TnlaEventHandler};
use crate::transport_provider::{ClientTransportProvider, TransportProvider};
use anyhow::{anyhow, Result};
use async_std::future;
use async_std::sync::Arc;
use async_std::task;
use async_trait::async_trait;
use serde_json;
use slog::{info, o, trace, warn, Logger};
use std::time::Duration;
use stop_token::StopToken;
use task::JoinHandle;

#[derive(Debug, Clone)]
pub struct SctpClientTransportProvider {
    tnla_pool: SctpTnlaPool,
    ppid: u32,
    use_json: bool,
}

impl SctpClientTransportProvider {
    pub fn new(ppid: u32, use_json: bool) -> SctpClientTransportProvider {
        SctpClientTransportProvider {
            tnla_pool: SctpTnlaPool::new(),
            ppid,
            use_json,
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
impl ClientTransportProvider for SctpClientTransportProvider {
    type Pdu = NgapPdu; // TODO
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
        let wrapped_handler = JsonDecoder(handler);
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

#[async_trait]
impl TransportProvider for SctpClientTransportProvider {
    type Pdu = NgapPdu;
    async fn send_pdu(&self, pdu: NgapPdu, logger: &Logger) -> Result<()> {
        let message: Vec<u8> = if self.use_json {
            trace!(logger, "Encode NGAP PDU as JSON");
            serde_json::to_string(&pdu).unwrap().into()
        } else {
            // The only real message we can send is an NG Setup
            trace!(logger, "Encode NGAP PDU as ASN.1");
            hex::decode("00150035000004001b00080002f83910000102005240090300667265653567630066001000000000010002f839000010080102030015400140").unwrap()
        };
        self.tnla_pool.send_message(message, logger).await
    }
}
