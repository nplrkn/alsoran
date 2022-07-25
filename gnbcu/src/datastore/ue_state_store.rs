use super::UeState;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait UeStateStore: Clone + Send + Sync + 'static {
    async fn store(&self, k: u32, s: UeState, ttl_secs: usize) -> Result<()>;
    async fn retrieve(&self, k: &u32) -> Result<UeState>;
    async fn delete(&self, k: &u32) -> Result<()>;
}
