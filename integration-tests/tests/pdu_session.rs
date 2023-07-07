mod test;
use anyhow::Result;
pub use test::*;

#[async_std::test]
async fn successful_pdu_session_setup_and_release() -> Result<()> {
    let mut tc = TestContextBuilder::new()
        .stage(Stage::DuConnected)
        .spawn()
        .await?;
    let ue = tc
        .create_and_register_ue(1)
        .await?
        .establish_pdu_session(&mut tc)
        .await?;

    ue.uplink_data_packet(&tc).await?;
    ue.downlink_data_packet(&tc).await?;

    let _ue = ue.release_pdu_session(&tc).await?;

    tc.terminate().await;
    Ok(())
}
