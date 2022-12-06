use anyhow::Result;
use common::{logging, signal};
use gnb_cu_cp_coordinator::Config;
use slog::info;

#[async_std::main]
async fn main() -> Result<()> {
    // Use info level logging by default
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    let root_logger = logging::init();
    let shutdown_handle = gnb_cu_cp_coordinator::spawn(Config::default(), root_logger.clone())?;
    let s = signal::wait_for_signal().await?;
    info!(root_logger, "Caught signal {} - terminate", s);
    shutdown_handle.graceful_shutdown().await;
    Ok(())
}
