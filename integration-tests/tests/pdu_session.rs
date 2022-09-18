mod test;
use anyhow::Result;
use async_std;
use serial_test::serial;
pub use test::*;

#[async_std::test]
#[serial]
async fn successful_pdu_session_setup() -> Result<()> {
    let tc = TestContext::new(Stage::Ue1Registered).await?;
    tc.establish_pdu_session(1).await?;
    tc.terminate().await;
    Ok(())
}
