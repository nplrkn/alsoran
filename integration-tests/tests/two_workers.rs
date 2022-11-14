mod test;
use anyhow::Result;
use async_std;
pub use test::*;

#[async_std::test]
async fn two_workers() -> Result<()> {
    // Set up two workers.
    let tc = TestContextBuilder::new()
        .worker_count(2)
        .stage(Stage::DuConnected)
        .spawn()
        .await?;

    // Register two UEs.  The UE ID differs by 1, and the test framework chooses the DU -> CU connection by (ID % num_workers),
    // so will send them to different workers.
    let _ue_1 = tc.create_and_register_ue(1).await?;
    let _ue_2 = tc.create_and_register_ue(2).await?;

    tc.terminate().await;
    Ok(())
}
