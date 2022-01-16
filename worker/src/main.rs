use also_net::Asn1PerCodec;
use anyhow::Result;
use common::logging;
use common::signal;
use slog::info;
use worker::config::Config;

#[async_std::main]
async fn main() -> Result<()> {
    let root_logger = logging::init();

    // TODO - get from command line.  (Note callback_server_bind_port is per instance so we
    // can't use a shared file unless the instances are dockerized.)
    let config = Config {
        callback_server_bind_port: 23256,
        callback_server_url_host_port: None,
    };

    let (stop_source, task) = worker::spawn(
        config,
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
