//! redis_ue_store - storage of UE state in Redis

use super::SerDes;
use anyhow::{Context, Result};
use async_trait::async_trait;
use redis::{AsyncCommands, Client};

use super::{StateStore, UeState, UeStateStore};

#[derive(Clone)]
pub struct RedisUeStore {
    client: Client,
}

impl RedisUeStore {
    pub fn new(port: u16) -> Result<Self> {
        let client = Client::open(format!("redis://127.0.0.1:{}/", port))?;
        Ok(RedisUeStore { client })
    }
}

#[async_trait]
impl StateStore<UeState> for RedisUeStore {
    async fn store(&self, k: u32, v: UeState, ttl_secs: usize) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        conn.set_ex(k, v.into_bytes()?, ttl_secs).await?;
        Ok(())
    }
    async fn retrieve(&self, k: &u32) -> Result<UeState> {
        let mut conn = self.client.get_async_connection().await?;
        let v: Vec<u8> = conn
            .get(k)
            .await
            .with_context(|| format!("Failed Redis get on UE {:#010x}?", k))?;
        Ok(UeState::from_bytes(&v)?)
    }
    async fn delete(&self, k: &u32) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        conn.del(k).await?;
        Ok(())
    }
}
impl UeStateStore for RedisUeStore {}
