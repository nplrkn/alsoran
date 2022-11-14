mod test;
use anyhow::Result;
use async_std;
pub use test::*;

#[async_std::test]
async fn cu_can_connect_to_amf() -> Result<()> {
    let tc = TestContextBuilder::new()
        .stage(Stage::AmfConnected)
        .spawn()
        .await?;
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
