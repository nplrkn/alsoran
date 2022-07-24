use super::UeState;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait UeStateStore: Clone + Send + Sync + 'static {
    async fn store(&self, k: u32, s: UeState, ttl_secs: u32) -> Result<()>;
    async fn retrieve(&self, k: &u32) -> Result<Option<UeState>>;
    async fn delete(&self, k: &u32) -> Result<()>;
}
