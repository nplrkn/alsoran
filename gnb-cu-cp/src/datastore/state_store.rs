use anyhow::Result;
use async_trait::async_trait;

pub trait SerDes: Sized {
    fn into_bytes(self) -> Result<Vec<u8>>;
    fn from_bytes(bytes: &[u8]) -> Result<Self>;
}

#[async_trait]
pub trait StateStore<T: SerDes>: Clone + Send + Sync + 'static {
    async fn store(&self, k: u32, s: T, ttl_secs: usize) -> Result<()>;
    async fn retrieve(&self, k: &u32) -> Result<T>;
    async fn delete(&self, k: &u32) -> Result<()>;
}
