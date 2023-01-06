use anyhow::Result;
use common::{logging, signal};
use gnb_cu_cp_coordinator::Config;
use slog::info;

#[async_std::main]
async fn main() -> Result<()> {
    let root_logger = logging::init();
    let shutdown_handle = gnb_cu_cp_coordinator::spawn(Config::default(), root_logger.clone())?;
    let s = signal::wait_for_signal().await?;
    info!(root_logger, "Caught signal {} - terminate", s);
    shutdown_handle.graceful_shutdown().await;
    Ok(())
}
