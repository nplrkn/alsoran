mod test;
use crate::Stage::*;
use anyhow::Result;
pub use mocks::Binding;
pub use test::*;

#[async_std::test]
async fn triangular_redirection() -> Result<()> {
    // Set up two workers.
    let tc = TestContextBuilder::new()
        .worker_count(2)
        .stage(DuConnected)
        .spawn()
        .await?;

    // Start with the UE initial access procedure.
    let mut ue_1 = tc.new_ue(1).await?.initial_access(&tc).await?;

    // This went through worker 0.  Have the AMF rebind to worker 1.
    let ngap_binding = &mut ue_1.0.amf_ue_context.binding;
    assert_eq!(&ngap_binding.remote_ip, &tc.worker_ip(0));
    tc.amf.rebind(ngap_binding, &tc.worker_ip(1)).await?;
    tc.du
        .rebind(&mut ue_1.0.du_ue_context.binding, &tc.worker_ip(1))
        .await?;

    // Complete registration through worker 2.
    ue_1.initiate_registration(&tc)
        .await?
        .complete_registration(&tc)
        .await?;

    // @@@ TODO can we impl Drop with block on instead of this?
    tc.terminate().await;
    Ok(())
}
