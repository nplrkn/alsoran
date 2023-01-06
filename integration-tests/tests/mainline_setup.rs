mod test;
use std::time::Duration;

use anyhow::Result;
pub use test::*;

#[async_std::test]
async fn cu_can_connect_to_amf() -> Result<()> {
    let tc = TestContextBuilder::new()
        .stage(Stage::AmfConnected)
        .spawn()
        .await?;

    // There's no good way to ensure that the NG Setup Response has been processed by the worker, so wait a moment before we
    // terminate.  The worker won't complete a graceful shutdown if its NG Setup exchange is still pending.
    async_std::task::sleep(Duration::from_millis(250)).await;

    tc.terminate().await;
    Ok(())
}

#[async_std::test]
async fn up_can_connect_to_cp() -> Result<()> {
    let tc = TestContextBuilder::new()
        .stage(Stage::CuUpConnected)
        .spawn()
        .await?;
    tc.terminate().await;
    Ok(())
}

#[async_std::test]
async fn du_can_connect_to_cu() -> Result<()> {
    let tc = TestContextBuilder::new()
        .stage(Stage::DuConnected)
        .spawn()
        .await?;
    tc.terminate().await;
    Ok(())
}

#[async_std::test]
async fn ue_can_register() -> Result<()> {
    let tc = TestContextBuilder::new()
        .stage(Stage::DuConnected)
        .spawn()
        .await?;
    let _ = tc.create_and_register_ue(1).await?;
    tc.terminate().await;
    Ok(())
}
