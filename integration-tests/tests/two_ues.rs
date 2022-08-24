mod test;
use anyhow::Result;
use async_std;
use serial_test::serial;
pub use test::*;

#[async_std::test]
#[serial]
async fn two_ues_register_sequentially() -> Result<()> {
    let mut tc = TestContext::new(Stage::Ue1Registered).await?;
    tc.register_ue(2).await?;
    tc.terminate().await;
    Ok(())
}
