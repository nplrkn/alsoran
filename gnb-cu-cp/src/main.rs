use anyhow::Result;
use common::{logging, signal};
use gnb_cu_cp::{Config, RedisUeStore};
use slog::info;

#[async_std::main]
async fn main() -> Result<()> {
    let root_logger = logging::init();
    let shutdown_handle = gnb_cu_cp::spawn(
        Config::default(),
        RedisUeStore::new(6379).unwrap(),
        root_logger.clone(),
    )?;
    let s = signal::wait_for_signal().await?;
    info!(root_logger, "Caught signal {} - terminate", s);
    shutdown_handle.graceful_shutdown().await;
    Ok(())
}
