mod test;
use anyhow::Result;
use async_std;
pub use test::*;

#[async_std::test]
async fn two_ues_register_sequentially() -> Result<()> {
    let mut tc = TestContextBuilder::new()
        .stage(Stage::Ue1Registered)
        .spawn()
        .await?;
    tc.register_ue(2).await.unwrap();
    tc.terminate().await;
    Ok(())
}

#[async_std::test]
async fn two_ues_register_in_parallel() -> Result<()> {
    let mut tc = TestContextBuilder::new()
        .stage(Stage::DuConnected)
        .spawn()
        .await?;

    let ue_1 = tc.register_ue_start(1);
    let ue_1 = tc.register_ue_next(ue_1).await?.unwrap();
    let ue_2 = tc.register_ue_start(2);
    let ue_2 = tc.register_ue_next(ue_2).await?.unwrap();

    // These will fail when more stages are added to Ue registration.
    assert!(tc.register_ue_next(ue_1).await?.is_none());
    assert!(tc.register_ue_next(ue_2).await?.is_none());

    Ok(())
}
