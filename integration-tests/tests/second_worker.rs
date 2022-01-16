mod test;
use anyhow::Result;
use async_std;
pub use test::*;

#[async_std::test]
async fn ran_configuration_update_for_second_worker() -> Result<()> {
    let mut test_context = TestContext::new().await;

    test_context.amf.expect_connection().await;
    test::ng_setup::handle(&test_context).await; // TODO change to amf.expect_connection(), amf.handle_ng_setup

    // Start a second worker.
    test_context.start_worker().await;
    test_context.amf.expect_connection().await;
    test::ran_configuration_update::handle(&test_context).await?;

    test_context.terminate().await;

    Ok(())
}
