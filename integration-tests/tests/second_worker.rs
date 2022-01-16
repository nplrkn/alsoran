mod test;
use anyhow::Result;
use async_std;
pub use test::*;

#[async_std::test]
async fn ran_configuration_update_for_second_worker() -> Result<()> {
    let mut tc = TestContext::new().await;
    tc.amf.expect_connection().await;
    tc.amf.handle_ng_setup(&tc.logger).await?;

    // Start a second worker.
    tc.start_worker().await;
    tc.amf.expect_connection().await;
    tc.amf.handle_ran_configuration_update(&tc.logger).await?;

    tc.terminate().await;

    Ok(())
}
