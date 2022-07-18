use super::UeState;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait UeStateStore {
    async fn store(&self, k: u64, s: UeState, ttl_secs: u32) -> Result<()>;
    async fn retrieve(&self, k: &u64) -> Result<Option<UeState>>;
    async fn delete(&self, k: &u64) -> Result<()>;
}
