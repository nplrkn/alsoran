mod test;
use crate::Stage::*;
use anyhow::Result;
use std::time::Duration;
pub use test::*;

#[async_std::test]
async fn two_workers() -> Result<()> {
    // Set up two workers.
    let tc = TestContextBuilder::new()
        .worker_count(2)
        .stage(DuConnected)
        .spawn()
        .await?;

    // Register two UEs.  The UE ID differs by 1, and the test framework chooses the DU -> CU connection by (ID % num_workers),
    // so will send them to different workers.
    let _ue_1 = tc.create_and_register_ue(1).await?;
    let _ue_2 = tc.create_and_register_ue(2).await?;

    tc.terminate().await;
    Ok(())
}

#[async_std::test]
async fn second_worker_gets_du_connection() -> Result<()> {
    // Set up NGAP and E1AP with the first worker going first.
    let mut tc = TestContextBuilder::new()
        .worker_count(2)
        .stage(CuUpConnected)
        .spawn()
        .await?;

    // Send the F1AP connection to the second worker and check the
    // first worker then gets added.
    tc.interface_setup_stage(1, &DuConnected, true).await?;
    tc.interface_setup_stage(0, &DuConnected, false).await?;

    tc.terminate().await;
    Ok(())
}

#[async_std::test]
async fn second_worker_gets_up_connection() -> Result<()> {
    // Set up NGAP with the first worker going first.
    let mut tc = TestContextBuilder::new()
        .worker_count(2)
        .stage(AmfConnected)
        .spawn()
        .await?;

    // Send the E1AP connection to the second worker and check the
    // first worker then gets added.
    tc.interface_setup_stage(1, &CuUpConnected, true).await?;
    tc.interface_setup_stage(0, &CuUpConnected, false).await?;

    // Setup F1AP.
    tc.interface_setup_stage(0, &DuConnected, true).await?;
    tc.interface_setup_stage(1, &DuConnected, false).await?;

    tc.terminate().await;
    Ok(())
}

#[async_std::test]
async fn two_workers_f1_before_e1() -> Result<()> {
    // Set up NGAP with the first worker going first.
    let mut tc = TestContextBuilder::new()
        .worker_count(2)
        .stage(AmfConnected)
        .spawn()
        .await?;

    // Setup F1AP.
    tc.interface_setup_stage(0, &DuConnected, true).await?;
    tc.interface_setup_stage(1, &DuConnected, false).await?;

    // Setup E1AP.
    tc.interface_setup_stage(1, &CuUpConnected, true).await?;
    tc.interface_setup_stage(0, &CuUpConnected, false).await?;

    tc.terminate().await;
    Ok(())
}

#[async_std::test]
async fn two_workers_f1_e1_interleaved() -> Result<()> {
    // Set up NGAP with the first worker going first.
    let mut tc = TestContextBuilder::new()
        .worker_count(2)
        .stage(AmfConnected)
        .spawn()
        .await?;

    // Setup F1AP on worker 1.
    tc.interface_setup_stage(0, &DuConnected, true).await?;

    // Setup E1AP on worker 2.  During this time, the coordinator will be
    // trying to get worker 2 to do F1 setup.
    tc.interface_setup_stage(1, &CuUpConnected, true).await?;

    // Complete F1AP setup with worker 2.
    tc.interface_setup_stage(1, &DuConnected, false).await?;

    // By this point, the coordinator has two queued up refreshes from worker 2, one without F1 and one with.
    // As per our connection management design, 'the approach taken is for the coordinator to enforce a delay between
    // any two attempts to connect a given worker'.  To do this, it stores a last attempt timestamp for both F1 and E1'.
    // In practice this means that it ignores the fact that worker 2 is claiming no F1 connection in its first refresh
    // and instead processes the E1 connection only.

    // Complete E1AP setup with worker 1.
    tc.interface_setup_stage(0, &CuUpConnected, false).await?;

    // The final procedure is initiated by the coordinator and there is currently no way to
    // wait till it has got a response.  So wait instead.  It might be nice to add a test message
    // that causes the coordinator to send a response from its control thread.
    async_std::task::sleep(Duration::from_millis(250)).await;

    tc.terminate().await;
    Ok(())
}
