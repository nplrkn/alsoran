//! main - starts a single-instance combined CU-CP and CU-UP

#![allow(unused_parens)]
use anyhow::Result;
use clap::Parser;
use common::{logging, panic, signal, ShutdownHandle};
use gnb_cu_cp::{Config as CpConfig, MockUeStore};
use gnb_cu_cp::{ConnectionControlConfig, ConnectionStyle};
use gnb_cu_up::Config as UpConfig;
use slog::{info, o, Logger};
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Local IP address to bind server ports to
    #[arg(long, default_value_t = IpAddr::V4(Ipv4Addr::UNSPECIFIED))]
    local_ip: IpAddr,

    /// AMF's NGAP IP address to connect to.
    #[arg(long, default_value_t = IpAddr::V4(Ipv4Addr::LOCALHOST))]
    amf_ip: IpAddr,

    /// PLMN ID.
    #[arg(long)]
    mcc: String,

    #[arg(long)]
    mnc: String,
}

#[async_std::main]
async fn main() -> Result<()> {
    panic::exit_on_panic();
    let args = Args::parse();
    let root_logger = logging::init();
    let cp_shutdown_handle = spawn_cp(&args, root_logger.new(o!("cu-cp" => 1)))?;

    // Wait a couple of seconds for the CP to bind its E1AP socket to avoid a retry and warning.
    async_std::task::sleep(Duration::from_secs(2)).await;

    let cu_shutdown_handle = spawn_up(&args, root_logger.new(o!("cu-up" => 1)))?;
    let s = signal::wait_for_signal().await?;
    info!(root_logger, "Caught signal {} - terminate", s);
    cp_shutdown_handle.graceful_shutdown().await;
    cu_shutdown_handle.graceful_shutdown().await;
    Ok(())
}

fn spawn_cp(args: &Args, logger: Logger) -> Result<ShutdownHandle> {
    if args.mcc.len() != 3 {
        panic!("MCC must be three decimal digits");
    }

    let mut plmn_digits = [0u8; 6];
    let mut plmn = [0u8; 3];
    for (n, c) in args.mcc.chars().enumerate() {
        let Some(digit) = c.to_digit(10) else {
            panic!("MCC must be three decimal digits");
        };
        plmn_digits[n] = digit as u8;
    }
    let offset = match args.mnc.len() {
        2 => {
            plmn_digits[3] = 0x0f;
            4
        }
        3 => 3,
        _ => panic!("MNC must be two or three digits"),
    };
    for (n, c) in args.mnc.chars().enumerate() {
        let Some(digit) = c.to_digit(10) else {
            panic!("MNC must be two or three digits")
        };
        plmn_digits[n + offset] = digit as u8;
    }
    for (n, digit) in plmn_digits.iter().enumerate() {
        let index = n / 2;
        plmn[index] = if (n % 2) == 0 {
            *digit
        } else {
            plmn[index] | (digit << 4)
        };
    }

    let cp_config = CpConfig {
        ip_addr: args.local_ip,
        connection_style: ConnectionStyle::Autonomous(ConnectionControlConfig {
            fast_start: true,
            amf_address: args.amf_ip.to_string(),
            worker_refresh_interval_secs: 10,
        }),
        plmn,
        ..CpConfig::default()
    };
    gnb_cu_cp::spawn(Uuid::new_v4(), cp_config, MockUeStore::new(), logger)
}

fn spawn_up(args: &Args, logger: Logger) -> Result<ShutdownHandle> {
    let cp_ip_address = if args.local_ip.is_unspecified() {
        IpAddr::V4(Ipv4Addr::LOCALHOST)
    } else {
        args.local_ip
    };
    let up_config = UpConfig {
        local_ip_address: args.local_ip,
        userplane_ip_address: args.local_ip,
        cp_ip_address,
        name: None,
    };

    gnb_cu_up::spawn(up_config, logger)
}
