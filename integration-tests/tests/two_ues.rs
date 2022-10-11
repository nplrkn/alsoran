mod test;
use anyhow::Result;
use async_std;
pub use test::*;

#[async_std::test]
async fn two_ues_register_sequentially() -> Result<()> {
    let mut tc = TestContext::new(Stage::Ue1Registered).await.unwrap();
    tc.register_ue(2).await.unwrap();
    tc.terminate().await;
    Ok(())
}
