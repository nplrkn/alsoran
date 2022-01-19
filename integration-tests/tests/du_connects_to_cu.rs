mod test;
use anyhow::Result;
use async_std;
pub use test::*;

#[async_std::test]
async fn du_can_connect() -> Result<()> {
    let tc = TestContext::new().await;

    // TODO to avoid boilerplate move into test context as procedure
    let amf = &tc.amf;
    let logger = &tc.logger;
    amf.expect_connection().await;
    amf.handle_ng_setup(logger).await?;

    let du = &tc.du;
    du.establish_connection(tc.worker_info(0).f1_host_port)
        .await?;
    du.perform_f1_setup().await?;

    tc.terminate().await;
    Ok(())
}
