use anyhow::Result;
use clap::Parser;
use common::{logging, signal};
use gnb_cu_cp::{Config, RedisUeStore};
use slog::info;
use std::net::IpAddr;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Optional local IP address to bind server ports to.
    #[arg(short, long)]
    local_ip: Option<IpAddr>,
}

#[async_std::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config {
        ip_addr: args.local_ip,
        ..Config::default()
    };
    let root_logger = logging::init();
    let shutdown_handle = gnb_cu_cp::spawn(
        Uuid::new_v4(),
        config,
        RedisUeStore::new(6379).unwrap(),
        root_logger.clone(),
    )?;
    let s = signal::wait_for_signal().await?;
    info!(root_logger, "Caught signal {} - terminate", s);
    shutdown_handle.graceful_shutdown().await;
    Ok(())
}
