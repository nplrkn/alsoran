use anyhow::Result;
use mocks::MockDu;

#[async_std::main]
async fn main() -> Result<()> {
    let logger = common::logging::init_terminal_logging();
    let (du, stop_source) = MockDu::new(&logger).await;

    du.establish_connection("127.0.0.1:38472".to_string())
        .await?;

    du.perform_rrc_setup(&logger).await?;

    let b = du.receive_nas().await?;
    println!("yay {:?}", b);

    drop(stop_source);

    Ok(())
}
