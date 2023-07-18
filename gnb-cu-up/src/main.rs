use anyhow::Result;
use common::{logging, panic, signal};
use gnb_cu_up::Config;
use slog::info;

#[async_std::main]
async fn main() -> Result<()> {
    panic::exit_on_panic();

    let root_logger = logging::init();
    let config = Config::default();
    let shutdown_handle = gnb_cu_up::spawn(config, root_logger.clone()).await?;
    let s = signal::wait_for_signal().await?;
    info!(root_logger, "Caught signal {} - terminate", s);
    shutdown_handle.graceful_shutdown().await;
    Ok(())
}
