mod test;
use anyhow::Result;
use async_std;
use serial_test::serial;
pub use test::*;

#[async_std::test]
#[serial]
async fn cu_can_connect_to_amf() -> Result<()> {
    let tc = TestContext::new(Stage::AmfConnected).await?;
    tc.terminate().await;
    Ok(())
}

#[async_std::test]
#[serial]
async fn du_can_connect_to_cu() -> Result<()> {
    let tc = TestContext::new(Stage::DuConnected).await?;
    tc.terminate().await;
    Ok(())
}

#[async_std::test]
#[serial]
async fn ue_can_register() -> Result<()> {
    let tc = TestContext::new(Stage::Ue1Registered).await?;
    tc.terminate().await;
    Ok(())
}
