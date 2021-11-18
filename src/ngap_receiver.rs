use crate::transport_provider::TransportMessageReceiver;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct NgapReceiver {}

#[async_trait]
impl TransportMessageReceiver for NgapReceiver {
    async fn recv_non_ue_associated_message(&self, buf: Vec<u8>) {
        unimplemented!();
    }
}
