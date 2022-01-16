mod test;
use anyhow::Result;
use async_std;
pub use test::*;

#[async_std::test]
async fn run_everything() -> Result<()> {
    let tc = TestContext::new().await;
    let amf = &tc.amf;
    let logger = &tc.logger;

    amf.expect_connection().await;
    amf.handle_ng_setup(logger).await?;

    tc.terminate().await;
    Ok(())
}
