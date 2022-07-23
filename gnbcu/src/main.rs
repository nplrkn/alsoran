use anyhow::Result;
use common::{logging, signal};
use gnbcu::{Config, Gnbcu, RedisUeStore};
use slog::info;

#[async_std::main]
async fn main() -> Result<()> {
    let root_logger = logging::init();
    let shutdown_handle = Gnbcu::spawn(Config::default(), RedisUeStore::new(), &root_logger)?;
    let s = signal::wait_for_signal().await?;
    info!(root_logger, "Caught signal {} - terminate", s);
    shutdown_handle.graceful_shutdown().await;
    Ok(())
}
