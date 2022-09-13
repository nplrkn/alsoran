//! redis_ue_store - storage of UE state in Redis

#![allow(dead_code, unused_variables)]
use anyhow::Result;
use async_trait::async_trait;
use redis::{AsyncCommands, Client};
use speedy::{Readable, Writable};

use super::{ue_state::UeStateSerializable, UeState, UeStateStore};

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
impl UeStateStore for RedisUeStore {
    async fn store(&self, k: u32, v: UeState, ttl_secs: usize) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        let v: UeStateSerializable = v.into();
        let v = v.write_to_vec()?;
        conn.set_ex(k, v, ttl_secs).await?;
        Ok(())
    }
    async fn retrieve(&self, k: &u32) -> Result<UeState> {
        let mut conn = self.client.get_async_connection().await?;
        let v: Vec<u8> = conn.get(k).await?;
        let v = UeStateSerializable::read_from_buffer(&v)?;
        Ok(v.into())
    }
    async fn delete(&self, k: &u32) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        conn.del(k).await?;
        Ok(())
    }
}
