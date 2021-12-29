use also_net::Asn1PerCodec;
use anyhow::Result;
use common::logging;
use common::signal;
use slog::info;

#[async_std::main]
async fn main() -> Result<()> {
    let root_logger = logging::init();
    let (stop_source, task) = worker::spawn(
        root_logger.clone(),
        Asn1PerCodec::new(),
        Asn1PerCodec::new(),
    );
    let s = signal::wait_for_signal().await?;
    info!(root_logger, "Caught signal {} - terminate", s);
    drop(stop_source);
    task.await;
    Ok(())
}
