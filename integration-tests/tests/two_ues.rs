mod test;
use anyhow::Result;
pub use test::*;

#[async_std::test]
async fn two_ues_register_sequentially() -> Result<()> {
    let tc = TestContextBuilder::new()
        .stage(Stage::DuConnected)
        .spawn()
        .await?;
    let _ue_1 = tc.create_and_register_ue(1).await?;
    let _ue_2 = tc.create_and_register_ue(2).await?;
    tc.terminate().await;
    Ok(())
}

#[async_std::test]
async fn two_ues_register_in_parallel() -> Result<()> {
    let tc = TestContextBuilder::new()
        .stage(Stage::DuConnected)
        .spawn()
        .await?;

    let ue_1 = tc
        .new_ue(1)
        .await?
        .initial_access(&tc)
        .await?
        .initiate_registration(&tc)
        .await?;

    let ue_2 = tc
        .new_ue(2)
        .await?
        .initial_access(&tc)
        .await?
        .initiate_registration(&tc)
        .await?;

    // Complete the initial context setup for both UEs.
    ue_2.complete_registration(&tc).await?;
    ue_1.complete_registration(&tc).await?;

    tc.terminate().await;
    Ok(())
}
