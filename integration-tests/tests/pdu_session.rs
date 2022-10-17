mod test;
use anyhow::Result;
use async_std;
pub use test::*;

#[async_std::test]
async fn successful_pdu_session_setup() -> Result<()> {
    let mut tc = TestContextBuilder::new()
        .stage(Stage::Ue1Registered)
        .spawn()
        .await?;
    tc.establish_pdu_session(1).await?;
    tc.terminate().await;
    Ok(())
}
