use anyhow::Result;
use common::{logging, signal};
use gnbcu::{Config, Gnbcu};
use slog::info;

#[async_std::main]
async fn main() -> Result<()> {
    let root_logger = logging::init();
    let (stop_source, task) = Gnbcu::spawn(Config::default(), &root_logger)?;
    let s = signal::wait_for_signal().await?;
    info!(root_logger, "Caught signal {} - terminate", s);
    drop(stop_source);
    task.await;
    Ok(())
}
