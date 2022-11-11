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
    tc.terminate().await;
    Ok(())
}
