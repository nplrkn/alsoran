mod test;
use anyhow::Result;
pub use test::*;

#[async_std::test]
async fn two_amf_endpoints_one_worker() -> Result<()> {
    let tc = TestContextBuilder::new()
        .stage(Stage::DuConnected)
        .amf_endpoint_count(2)
        .spawn()
        .await?;
    let _ue_1 = tc.create_and_register_ue(1).await?;
    let _ue_2 = tc.create_and_register_ue(2).await?;
    tc.terminate().await;
    Ok(())
}
