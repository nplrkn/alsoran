mod test;
use anyhow::Result;
pub use test::*;

#[async_std::test]
async fn gnb_du_configuration_update() -> Result<()> {
    let tc = TestContextBuilder::new()
        .stage(Stage::DuConnected)
        .spawn()
        .await?;

    tc.du.perform_du_configuration_update().await?;

    tc.terminate().await;
    Ok(())
}
