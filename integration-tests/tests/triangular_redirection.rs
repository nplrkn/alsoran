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
    let mut ue_1 = tc.new_ue(1).await;
    let mut amf_ue_context = tc.ue_initial_access(&mut ue_1).await?;

    // This went through worker 1.
    assert_eq!(amf_ue_context.binding_remote_ip(), tc.worker_ip(1));

    // Continue and complete Ue registration through worker 2.
    amf_ue_context.rebind(tc.worker_ip(2));
    tc.register_ue_next(amf_ue_context).await?;

    // @@@ can we impl Drop instead of this?
    tc.terminate().await;
    Ok(())
}
