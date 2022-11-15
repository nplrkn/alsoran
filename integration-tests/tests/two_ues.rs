mod test;
use anyhow::Result;
use async_std;
pub use test::*;
use test_context::UeRegisterStage;

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

    let mut ue_1 = tc.new_ue(1).await;

    // Get to an the initial context setup requests for both UEs
    let mut registration_1 = tc.register_ue_start(&mut ue_1).await;
    registration_1 = tc.register_ue_next(registration_1).await?;
    let mut ue_2 = tc.new_ue(2).await;
    let mut registration_2 = tc.register_ue_start(&mut ue_2).await;
    registration_2 = tc.register_ue_next(registration_2).await?;

    // Complete the initial context setup for both UEs.
    registration_2 = tc.register_ue_next(registration_2).await?;
    registration_1 = tc.register_ue_next(registration_1).await?;

    // These will fail when more stages are added to Ue registration.
    assert!(matches!(registration_1.stage, UeRegisterStage::End));
    assert!(matches!(registration_2.stage, UeRegisterStage::End));

    tc.terminate().await;
    Ok(())
}
