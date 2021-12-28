use crate::ngap::NgapPdu;
use crate::sctp;
use crate::sctp_tnla_pool::SctpTnlaPool;
use crate::tnla_event_handler::{JsonDecoder, TnlaEventHandler};
use crate::transport_provider::{ServerTransportProvider, TransportProvider};
use anyhow::Result;
use async_std::sync::Arc;
use async_std::task;
use async_std::task::JoinHandle;
use async_trait::async_trait;
use futures::stream::StreamExt;
use futures::{future, pin_mut};
use slog::{info, o, trace, Logger};
use stop_token::StopToken;

#[derive(Debug, Clone)]
pub struct SctpServerTransportProvider {
    tnla_pool: SctpTnlaPool,
    ppid: u32,
    use_json: bool,
}

impl SctpServerTransportProvider {
    pub fn new(ppid: u32, use_json: bool) -> SctpServerTransportProvider {
        SctpServerTransportProvider {
            tnla_pool: SctpTnlaPool::new(),
            ppid,
            use_json,
        }
    }
}

#[async_trait]
impl TransportProvider for SctpServerTransportProvider {
    type Pdu = NgapPdu;
    async fn send_pdu(&self, pdu: NgapPdu, logger: &Logger) -> Result<()> {
        let message: Vec<u8> = if self.use_json {
            trace!(logger, "JSON encode NGAP PDU");
            serde_json::to_string(&pdu).unwrap().into()
        } else {
            // The only real message we can send as a server is an NG Setup response
            trace!(logger, "ASN.1 encode NGAP PDU");
            hex::decode("20150031000004000100050100414d4600600008000002f839cafe0000564001ff005000100002f83900011008010203100811223300").unwrap()
        };
        self.tnla_pool.send_message(message, logger).await
    }
}

const MAX_LISTEN_BACKLOG: i32 = 5;

#[async_trait]
impl ServerTransportProvider for SctpServerTransportProvider {
    type Pdu = NgapPdu;
    async fn serve<H>(
        self,
        listen_addr: String,
        stop_token: StopToken,
        handler: H,
        logger: Logger,
    ) -> Result<JoinHandle<()>>
    where
        H: TnlaEventHandler<MessageType = <Self as ServerTransportProvider>::Pdu>,
    {
        let addr = async_net::resolve(listen_addr)
            .await
            .map(|vec| vec[0])?
            .into();

        let wrapped_handler = JsonDecoder(handler);

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
