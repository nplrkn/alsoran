mod test;
use anyhow::Result;
use async_std;
pub use test::*;

#[async_std::test]
async fn second_worker_connects_to_amf() -> Result<()> {
    let mut tc = TestContext::new().await;

    // The first worker initializes the NG interface.
    tc.amf.expect_connection().await;
    tc.amf.handle_ng_setup(&tc.logger).await?;

    // The second worker sends a RAN configuration update.
    tc.start_worker().await;
    tc.amf.expect_connection().await;
    tc.amf.handle_ran_configuration_update(&tc.logger).await?;

    tc.terminate().await;

    Ok(())
}
