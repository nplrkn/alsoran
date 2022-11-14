mod test;
use anyhow::Result;
use async_std;
pub use test::*;

#[async_std::test]
async fn two_workers() -> Result<()> {
    // Fully set up a first worker.
    let tc = TestContextBuilder::new()
        .worker_count(2)
        .stage(Stage::DuConnected)
        .spawn()
        .await?;

    let ue_1 = tc.register_ue_start(1);
    let ue_1 = tc.register_ue_next(ue_1).await?.unwrap();

    tc.terminate().await;
    Ok(())
}
