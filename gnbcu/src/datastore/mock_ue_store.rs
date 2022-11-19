//! mock_ue_store - allows testing of UE stateful operations without needing to run a real datastore

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use async_std::sync::{Arc, Mutex};
use async_trait::async_trait;

use super::{UeState, UeStateStore};

#[derive(Clone, Debug)]
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

impl Default for MockUeStore {
    fn default() -> Self {
        Self::new()
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
            .cloned()
            .ok_or_else(|| anyhow!("No such key)"))
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
        let ue_state = UeState::new(GnbDuUeF1apId(3));
        let key = ue_state.key;
        m.store(key, ue_state, 0).await?;
        let _ue_state = m.retrieve(&key).await.unwrap();
        assert!(m.retrieve(&0).await.is_err());
        m.delete(&key).await.unwrap();
        assert!(m.retrieve(&key).await.is_err());
        m.delete(&0).await.unwrap();
        Ok(())
    }
}
