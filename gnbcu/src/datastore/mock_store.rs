use std::collections::HashMap;

use anyhow::Result;
use async_std::sync::{Arc, Mutex};
use async_trait::async_trait;

use super::{UeState, UeStateStore};

pub struct MockStore {
    kvs: Arc<Mutex<HashMap<u64, UeState>>>,
}

impl MockStore {
    pub fn new() -> Self {
        MockStore {
            kvs: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl UeStateStore for MockStore {
    async fn store(&self, k: u64, s: UeState, _ttl_secs: u32) -> Result<()> {
        self.kvs.lock().await.insert(k, s);
        Ok(())
    }
    async fn retrieve(&self, k: &u64) -> Result<Option<UeState>> {
        Ok(self.kvs.lock().await.get(k).map(|x| x.clone()))
    }
    async fn delete(&self, k: &u64) -> Result<()> {
        self.kvs.lock().await.remove(k);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use f1ap::{GnbCuUeF1apId, GnbDuUeF1apId};

    use crate::datastore::UeState;

    use super::*;

    #[async_std::test]
    async fn test_mock_store() -> Result<()> {
        let m = MockStore::new();
        let ue_state = UeState {
            gnb_du_ue_f1ap_id: GnbDuUeF1apId(3),
            gnb_cu_ue_f1ap_id: GnbCuUeF1apId(2),
        };
        m.store(123, ue_state, 0).await?;
        let _ue_state = m.retrieve(&123).await.unwrap().unwrap();
        assert!(m.retrieve(&0).await.unwrap().is_none());
        m.delete(&123).await.unwrap();
        assert!(m.retrieve(&123).await.unwrap().is_none());
        m.delete(&0).await.unwrap();
        Ok(())
    }
}
