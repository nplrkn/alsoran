mod ue;
use anyhow::Result;
use mocks::MockDu;
use slog::info;
use ue::Ue;

#[async_std::main]
async fn main() -> Result<()> {
    let logger = common::logging::init_terminal_logging();
    let (du, stop_source) = MockDu::new(&logger).await;

    du.establish_connection("127.0.0.1:38472".to_string())
        .await?;

    let mut ue = Ue::new();

    info!(&logger, "RRC Setup with NAS Registration Request");
    let nas_message = ue.recv_nas();
    du.perform_rrc_setup(nas_message, &logger).await?;

    let nas_authentication_request = du.receive_nas().await?;
    info!(&logger, "<- NAS Authentication request --");
    ue.send_nas(nas_authentication_request, &logger);
    let nas_message = ue.recv_nas();
    info!(&logger, "-- NAS Authentication response ->");
    du.send_nas(nas_message, &logger).await?;

    assert!(false);

    drop(stop_source);

    Ok(())
}
