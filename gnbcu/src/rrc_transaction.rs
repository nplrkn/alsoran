use anyhow::Result;
use async_channel::{Receiver, Sender};
use async_std::sync::Mutex;
use rrc::UlDcchMessage;
use std::{collections::HashMap, sync::Arc};

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
pub struct PendingRrcTransactions(Arc<Mutex<HashMap<u64, Sender<UlDcchMessage>>>>);

impl PendingRrcTransactions {
    pub fn new() -> Self {
        PendingRrcTransactions(Arc::new(Mutex::new(HashMap::new())))
    }
    pub async fn new_transaction(&self, ue_id: u64) -> RrcTransaction {
        let (sender, receiver) = async_channel::bounded::<UlDcchMessage>(1);
        self.0.lock().await.insert(ue_id, sender);
        RrcTransaction { receiver }
    }
    pub async fn match_transaction(&self, ue_id: u64) -> Option<Sender<UlDcchMessage>> {
        self.0.lock().await.remove(&ue_id)
    }
}
