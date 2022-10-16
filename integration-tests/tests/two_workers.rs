mod test;
use anyhow::Result;
use async_std;
pub use test::*;

#[async_std::test]
async fn two_workers() -> Result<()> {
    let tc = TestContextBuilder::new().worker_count(2).spawn().await?;
    tc.terminate().await;
    Ok(())
}
