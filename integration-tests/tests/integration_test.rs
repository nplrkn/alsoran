mod test;
use async_std;
pub use test::*;

#[async_std::test]
async fn run_everything() {
    let test_context = TestContext::new().await;

    test_context.amf.expect_connection().await;
    test::ng_setup::handle(&test_context).await;

    test_context.terminate().await;
}
