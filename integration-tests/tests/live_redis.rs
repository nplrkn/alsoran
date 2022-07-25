mod test;
use anyhow::Result;
use async_std;
use serial_test::serial;
pub use test::*;

#[async_std::test]
#[serial]
async fn ue_can_register() -> Result<()> {
    let tc = TestContext::new_with_redis(Stage::UeRegistered).await?;
    tc.terminate().await;
    Ok(())
}
