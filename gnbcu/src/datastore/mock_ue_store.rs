use std::collections::HashMap;

use anyhow::{anyhow, Result};
use async_std::sync::{Arc, Mutex};
use async_trait::async_trait;

use super::{UeState, UeStateStore};

#[derive(Clone)]
pub struct MockUeStore {
    kvs: Arc<Mutex<HashMap<u32, UeState>>>,
}

impl MockUeStore {
    pub fn new() -> Self {
        MockUeStore {
            kvs: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl UeStateStore for MockUeStore {
    async fn store(&self, k: u32, s: UeState, _ttl_secs: usize) -> Result<()> {
        self.kvs.lock().await.insert(k, s);
        Ok(())
    }
    async fn retrieve(&self, k: &u32) -> Result<UeState> {
        self.kvs
            .lock()
            .await
            .get(k)
            .map(|x| x.clone())
            .ok_or(anyhow!("No such key)"))
    }
    async fn delete(&self, k: &u32) -> Result<()> {
        self.kvs.lock().await.remove(k);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use f1ap::GnbDuUeF1apId;

    use crate::datastore::UeState;

    use super::*;

    #[async_std::test]
    async fn test_mock_store() -> Result<()> {
        let m = MockUeStore::new();
        let ue_state = UeState {
            amf_ue_ngap_id: None,
            gnb_du_ue_f1ap_id: GnbDuUeF1apId(3),
            key: 2,
        };
        m.store(123, ue_state, 0).await?;
        let _ue_state = m.retrieve(&123).await.unwrap();
        assert!(m.retrieve(&0).await.is_err());
        m.delete(&123).await.unwrap();
        assert!(m.retrieve(&123).await.is_err());
        m.delete(&0).await.unwrap();
        Ok(())
    }
}
