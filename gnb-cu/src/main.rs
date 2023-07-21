//! main - starts a single-instance combined CU-CP and CU-UP

#![allow(unused_parens)]
use anyhow::{bail, ensure, Result};
use clap::Parser;
use common::{logging, panic, signal, ShutdownHandle};
use coordinator::Config as CoordinatorConfig;
use gnb_cu_cp::{Config as CpConfig, MockUeStore, WorkerConnectionManagementConfig};
use gnb_cu_cp::{ConnectionControlConfig, ConnectionStyle};
use gnb_cu_up::Config as UpConfig;
use slog::{info, o, warn, Logger};
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Local IP address of GNB-CU, both for control plane (F1AP, E1AP, NGAP) and userplane protocols (GTP-U).
    #[arg(long, default_value_t = IpAddr::V4(Ipv4Addr::UNSPECIFIED))]
    local_ip: IpAddr,

    /// AMF's NGAP IP address to connect to.
    #[arg(long, default_value_t = IpAddr::V4(Ipv4Addr::LOCALHOST))]
    amf_ip: IpAddr,

    /// Mobile Country Code part of the PLMN ID (Public Land Mobile Network ID).  
    /// A string of three digits.
    #[arg(long)]
    mcc: String,

    /// Mobile Network Code part of the PLMN ID (Public Land Mobile Network ID).  
    /// A string of two or three digits.
    #[arg(long)]
    mnc: String,
}

const CONNECTION_API_PORT: u16 = 50312;
const COORDINATION_API_PORT: u16 = 65232;

#[async_std::main]
async fn main() -> Result<()> {
    panic::exit_on_panic();
    let args = Args::parse();
    let root_logger = logging::init();

    // Attempt to bind a new coordinator to 0.0.0.0:65232.
    let maybe_coordinator = spawn_coordinator(&args, root_logger.new(o!("coord" => 1))).await;

    let (cp_shutdown_handle, local_ip) = spawn_cp(&args, root_logger.new(o!("cu-cp" => 1))).await?;

    // Wait a couple of seconds for the CP to bind its E1AP socket to avoid a retry and warning.
    async_std::task::sleep(Duration::from_secs(2)).await;

    let cu_shutdown_handle = spawn_up(local_ip, root_logger.new(o!("cu-up" => 1))).await?;
    let s = signal::wait_for_signal().await?;
    info!(root_logger, "Caught signal {} - terminate", s);
    cp_shutdown_handle.graceful_shutdown().await;
    cu_shutdown_handle.graceful_shutdown().await;
    if let Ok(handle) = maybe_coordinator {
        handle.graceful_shutdown().await;
    }

    Ok(())
}

async fn spawn_coordinator(args: &Args, logger: Logger) -> Result<ShutdownHandle> {
    let maybe_coordinator = coordinator::spawn(
        CoordinatorConfig {
            bind_port: COORDINATION_API_PORT,
            connection_control_config: ConnectionControlConfig {
                amf_address: args.amf_ip.to_string(),
                worker_refresh_interval_secs: 10,
                fast_start: true,
            },
        },
        logger.clone(),
    );
    match maybe_coordinator {
        Ok(_) => info!(logger, "This instance is acting as coordinator"),
        Err(_) => info!(logger, "Another instance is already acting as coordinator"),
    };
    maybe_coordinator
}

async fn spawn_cp(args: &Args, logger: Logger) -> Result<(ShutdownHandle, IpAddr)> {
    let plmn = convert_mcc_mnc_to_plmn_array(&args.mcc, &args.mnc).unwrap();
    let ip_addr = args.local_ip;
    let cp_config = CpConfig {
        ip_addr,
        connection_style: ConnectionStyle::Coordinated(WorkerConnectionManagementConfig {
            connection_api_bind_port: CONNECTION_API_PORT, // TODO - make configurable
            connection_api_base_path: format!("http://{ip_addr}:{CONNECTION_API_PORT}"),
            coordinator_base_path: format!("http://127.0.0.1:{COORDINATION_API_PORT}"),
        }),
        plmn,
        ..CpConfig::default()
    };
    gnb_cu_cp::spawn(
        Uuid::new_v4(),
        cp_config,
        MockUeStore::new(),
        logger.clone(),
    )
    .await
    .map(|h| (h, ip_addr))
}

async fn spawn_up(local_ip: IpAddr, logger: Logger) -> Result<ShutdownHandle> {
    let up_config = UpConfig {
        local_ip_address: local_ip,
        userplane_ip_address: local_ip,
        cp_ip_address: local_ip,
        name: None,
    };

    gnb_cu_up::spawn(up_config, logger).await
}

fn convert_mcc_mnc_to_plmn_array(mcc: &str, mnc: &str) -> Result<[u8; 3]> {
    ensure!(mcc.len() == 3, "MCC must be three decimal digits");

    let mut plmn_digits = [0u8; 6];
    let mut plmn = [0u8; 3];
    for (n, c) in mcc.chars().enumerate() {
        let Some(digit) = c.to_digit(10) else {
            bail!("MCC must be three decimal digits");
        };
        plmn_digits[n] = digit as u8;
    }
    let offset = match mnc.len() {
        2 => {
            plmn_digits[3] = 0x0f;
            4
        }
        3 => 3,
        _ => bail!("MNC must be two or three digits"),
    };
    for (n, c) in mnc.chars().enumerate() {
        let Some(digit) = c.to_digit(10) else {
            bail!("MNC must be two or three digits")
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
    Ok(plmn)
}

// This code is currently unused but provides a convenient way to run multiple local GNB-CUs
// without needing to specify IP addresses.  This is only useful if
// both 5GC and GNB-DU are running in the same network namespace as the GNB-CUs
// (and hence can contact localhost addresses).
#[allow(dead_code)]
async fn spawn_cp_on_random_local_addr(
    args: &Args,
    logger: Logger,
) -> Result<(ShutdownHandle, IpAddr)> {
    let plmn = convert_mcc_mnc_to_plmn_array(&args.mcc, &args.mnc).unwrap();

    // If no local address is specified on the command line, we search for one, starting at 127.0.0.1.
    // For each address we will try to bind the SCTP ports.
    for attempt in 1..10 {
        let ip_addr = IpAddr::from(Ipv4Addr::new(127, 0, 0, attempt));
        let cp_config = CpConfig {
            ip_addr,
            connection_style: ConnectionStyle::Coordinated(WorkerConnectionManagementConfig {
                connection_api_bind_port: CONNECTION_API_PORT, // TODO - make configurable
                connection_api_base_path: format!("http://{ip_addr}:{CONNECTION_API_PORT}"),
                coordinator_base_path: format!("http://127.0.0.1:{COORDINATION_API_PORT}"),
            }),
            plmn,
            ..CpConfig::default()
        };
        match gnb_cu_cp::spawn(
            Uuid::new_v4(),
            cp_config,
            MockUeStore::new(),
            logger.clone(),
        )
        .await
        {
            Ok(x) => return Ok((x, ip_addr)),
            Err(e) => {
                if !args.local_ip.is_unspecified() {
                    return Err(e);
                }
                warn!(
                    logger,
                    "Local address {ip_addr} already in use, presumably by another worker - retry with the next one"
                )
            }
        }
    }
    bail!("Failed to bind after multiple attempts")
}
