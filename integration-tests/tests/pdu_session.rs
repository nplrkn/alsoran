mod test;
use anyhow::Result;
pub use test::*;

#[async_std::test]
async fn successful_pdu_session_setup() -> Result<()> {
    let mut tc = TestContextBuilder::new()
        .stage(Stage::DuConnected)
        .spawn()
        .await?;
    let _ue = tc
        .create_and_register_ue(1)
        .await?
        .establish_pdu_session(&mut tc)
        .await?;
    tc.terminate().await;
    Ok(())
}
