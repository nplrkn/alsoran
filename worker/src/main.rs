use anyhow::Result;
use common::logging;
use common::signal;
use slog::info;

#[async_std::main]
async fn main() -> Result<()> {
    let root_logger = logging::init();
    let stop_source = worker::run(root_logger.clone()).await;
    let s = signal::wait_for_signal().await?;
    info!(root_logger, "Caught signal {} - terminate", s);
    drop(stop_source);
    Ok(())
}
