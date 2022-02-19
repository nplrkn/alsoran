mod test;
use anyhow::Result;
use async_std;
pub use test::*;

#[async_std::test]
async fn second_worker_INCOMPLETE() -> Result<()> {
    let mut tc = TestContext::new().await;

    // The first worker initializes the NG interface.
    tc.amf.expect_connection().await;
    tc.amf.handle_ng_setup(&tc.logger).await?;

    // The DU attaches to it.
    let du = &tc.du;
    du.establish_connection(tc.worker_info(0).f1ap_host_port)
        .await?;
    du.perform_f1_setup().await?;

    // The second worker sends a RAN configuration update.
    tc.start_worker().await;
    tc.amf.expect_connection().await;
    tc.amf.handle_ran_configuration_update(&tc.logger).await?;

    // The second worker then shares its F1AP server port with the first worker via the coordinator
    // and the first worker asks the DU to connect to the new address.
    //du.handle_gnbcu_tnla_addition(&tc.logger).await?;

    tc.terminate().await;

    Ok(())
}
