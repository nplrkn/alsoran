use anyhow::Result;
use clap::Parser;
use common::{logging, panic, signal};
use coordinator::ConnectionControlConfig;
use gnb_cu_cp::{Config, ConnectionStyle, RedisUeStore};
use slog::info;
use std::net::{IpAddr, Ipv4Addr};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Local IP address to bind server ports to (E1AP and F1AP).
    #[arg(short, long, default_value_t = IpAddr::V4(Ipv4Addr::UNSPECIFIED))]
    local_ip: IpAddr,

    /// AMF's NGAP IP address to connect to.
    #[arg(short, long, default_value_t = IpAddr::V4(Ipv4Addr::LOCALHOST))]
    amf_ip: IpAddr,
}

#[async_std::main]
async fn main() -> Result<()> {
    panic::exit_on_panic();

    let args = Args::parse();
    let config = Config {
        ip_addr: args.local_ip,
        connection_style: ConnectionStyle::Autonomous(ConnectionControlConfig {
            fast_start: true,
            amf_address: args.amf_ip.to_string(),
            ..ConnectionControlConfig::default()
        }),

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
