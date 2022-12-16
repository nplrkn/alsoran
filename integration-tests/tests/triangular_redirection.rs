mod test;
use crate::Stage::*;
use anyhow::Result;
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
    let mut ue_1 = tc.new_ue(1).await.initial_access(&tc).await?;

    // This went through worker 1.
    let amf_ue_context = ue_1.amf_ue_context();
    assert_eq!(amf_ue_context.binding_remote_ip(), &tc.worker_ip(1));

    // Continue and complete Ue registration through worker 2.
    tc.amf.rebind_ue_context(amf_ue_context, &tc.worker_ip(2));
    let (ue_1, security_mode_command) = ue_1.initiate_registration(&tc).await?;

    // @@@ TODO - want to be able to chain this too, so store the security mode command in the HalfRegisteredUe.

    ue_1.complete_registration(&tc, &security_mode_command)
        .await?;

    // @@@ TODO can we impl Drop instead of this?
    tc.terminate().await;
    Ok(())
}
