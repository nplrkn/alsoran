mod test;
use crate::Stage::*;
use anyhow::Result;
pub use mocks::Binding;
pub use test::*;

#[async_std::test]
async fn triangular_redirection() -> Result<()> {
    // Set up two workers.
    let tc = TestContextBuilder::new()
        .worker_count(2)
        .stage(DuConnected)
        .spawn()
        .await?;

    // Start with the UE initial access procedure.  This goes through worker 0.
    let mut ue_1 = tc.new_ue(1).await?.initial_access(&tc).await?;

    // Switch to worker 1.
    tc.use_worker_for_ue(1, &mut ue_1).await?;

    // Complete registration.
    ue_1.initiate_registration(&tc)
        .await?
        .complete_registration(&tc)
        .await?;

    tc.terminate().await;
    Ok(())
}
