mod test;
use anyhow::Result;
use std::time::Duration;
pub use test::*;

#[async_std::test]
async fn amf_status_indication() -> Result<()> {
    let tc = TestContextBuilder::new()
        .stage(Stage::AmfConnected)
        .spawn()
        .await?;
    tc.amf.send_status_indication().await?;

    // There's no good way to observe this, so wait a moment before we tear down the TNLA to give time for the
    // message to be processed.
    async_std::task::sleep(Duration::from_millis(250)).await;

    tc.terminate().await;
    Ok(())
}
