#![allow(dead_code, unused_variables)]
use anyhow::Result;
use async_trait::async_trait;

use super::{UeState, UeStateStore};

pub struct RedisUeStore {}

impl RedisUeStore {
    pub fn new() -> Self {
        todo!()
    }
}

#[async_trait]
impl UeStateStore for RedisUeStore {
    async fn store(&self, k: u64, s: UeState, _ttl_secs: u32) -> Result<()> {
        todo!()
    }
    async fn retrieve(&self, k: &u64) -> Result<Option<UeState>> {
        todo!()
    }
    async fn delete(&self, k: &u64) -> Result<()> {
        todo!()
    }
}
