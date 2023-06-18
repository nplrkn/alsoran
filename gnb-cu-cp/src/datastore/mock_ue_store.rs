//! mock_ue_store - allows testing of UE stateful operations without needing to run a real datastore

use super::{StateStore, UeState, UeStateStore};
use anyhow::{anyhow, Result};
use async_std::sync::Arc;
use async_trait::async_trait;
use dashmap::DashMap;

#[derive(Clone, Debug)]
pub struct MockUeStore {
    kvs: Arc<DashMap<u32, UeState>>,
}

impl MockUeStore {
    pub fn new() -> Self {
        MockUeStore {
            kvs: Arc::new(DashMap::new()),
        }
    }
}

impl Default for MockUeStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl StateStore<UeState> for MockUeStore {
    async fn store(&self, k: u32, s: UeState, _ttl_secs: usize) -> Result<()> {
        self.kvs.insert(k, s);
        Ok(())
    }
    async fn retrieve(&self, k: &u32) -> Result<UeState> {
        self.kvs
            .get(k)
            .ok_or_else(|| anyhow!("No such key"))
            .map(|x| x.clone())
    }
    async fn delete(&self, k: &u32) -> Result<()> {
        self.kvs.remove(k);
        Ok(())
    }
}
impl UeStateStore for MockUeStore {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datastore::UeState;
    use bitvec::prelude::*;
    use f1ap::GnbDuUeF1apId;

    #[async_std::test]
    async fn test_mock_store() -> Result<()> {
        let m = MockUeStore::new();
        let ue_state = UeState::new(
            GnbDuUeF1apId(3),
            f1ap::NrCgi {
                plmn_identity: f1ap::PlmnIdentity([2, 3, 2]),
                nr_cell_identity: f1ap::NrCellIdentity(bitvec![u8,Msb0;0;36]),
            },
        );
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
