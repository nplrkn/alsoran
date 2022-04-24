use super::{SharedTransactions, TransportProvider};
use anyhow::Result;
use asn1_codecs::aper::{AperCodec, AperCodecData};
use async_channel;
use async_trait::async_trait;
use sctp::Message;
use slog::Logger;
use xxap_transaction::{IntoPdu, Procedure, RequestError, RequestProvider};

#[derive(Clone)]
pub struct TransactionSender<T>
where
    T: TransportProvider,
{
    pending_requests: SharedTransactions,
    pub transport_provider: T,
}

impl<T> TransactionSender<T>
where
    T: TransportProvider,
{
    pub fn new(transport_provider: T, transactions: SharedTransactions) -> Self {
        TransactionSender {
            pending_requests: transactions,
            transport_provider,
        }
    }
}

#[async_trait]
impl<P: Procedure, Trans: TransportProvider + Sync + Send> RequestProvider<P>
    for TransactionSender<Trans>
{
    async fn request(
        &self,
        r: P::Request,
        logger: &Logger,
    ) -> Result<P::Success, RequestError<P::Failure>> {
        let mut d = AperCodecData::new();
        r.into_pdu().encode(&mut d)?;
        let bytes = d.into_bytes();

        // Create a channel to receive the response.
        let (sender, receiver) = async_channel::bounded::<Vec<u8>>(1);
        let match_fn = |m: &Message| ((m[0] != 0) && (m[1] == P::CODE));
        self.pending_requests
            .lock()
            .await
            .push((Box::new(match_fn), sender));

        self.transport_provider.send_message(bytes, logger).await?;
        let msg = receiver.recv().await?;

        let mut d = AperCodecData::from_slice(&msg);
        if msg[0] == 1 {
            Ok(P::Success::decode(&mut d)?)
        } else {
            Ok(P::Success::decode(&mut d)?) // TODO unsuccess
        }
    }
}
