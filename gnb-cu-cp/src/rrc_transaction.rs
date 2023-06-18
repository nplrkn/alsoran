//! rrc_transaction - enables workflow business logic to await a response to its Rrc request

use anyhow::Result;
use async_channel::{Receiver, Sender};
use dashmap::DashMap;
use rrc::UlDcchMessage;
use std::sync::Arc;

pub struct RrcTransaction {
    receiver: Receiver<UlDcchMessage>,
}

impl RrcTransaction {
    pub async fn recv(self) -> Result<UlDcchMessage> {
        let m = self.receiver.recv().await?;
        Ok(m)
    }
}

#[derive(Clone)]
pub struct PendingRrcTransactions(Arc<DashMap<u32, Sender<UlDcchMessage>>>);

impl PendingRrcTransactions {
    pub fn new() -> Self {
        PendingRrcTransactions(Arc::new(DashMap::new()))
    }
    pub async fn new_transaction(&self, ue_id: u32) -> RrcTransaction {
        let (sender, receiver) = async_channel::bounded::<UlDcchMessage>(1);
        self.0.insert(ue_id, sender);
        RrcTransaction { receiver }
    }
    pub async fn match_transaction(&self, ue_id: u32) -> Option<Sender<UlDcchMessage>> {
        self.0.remove(&ue_id).map(|x| x.1)
    }
}
