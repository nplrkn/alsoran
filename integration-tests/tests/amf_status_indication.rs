mod test;
use std::time::Duration;

use anyhow::Result;
use async_std;
use serial_test::serial;
pub use test::*;

#[async_std::test]
#[serial]
async fn amf_status_indication() -> Result<()> {
    let tc = TestContext::new(Stage::AmfConnected).await?;
    tc.amf.send_status_indication().await?;

    // There's no good way to observe this, so wait a moment before we tear down the TNLA to give time for the
    // message to be processed.
    async_std::task::sleep(Duration::from_millis(250)).await;

    tc.terminate().await;
    Ok(())
}