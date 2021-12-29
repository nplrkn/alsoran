mod test;
use async_std;
use test::TestContext;

#[async_std::test]
async fn run_everything() {
    let test_context = TestContext::new().await;
    let logger = &test_context.logger;
    let amf = &test_context.amf;

    // Wait for connection to be established - the mock sends us an empty message to indicate this.
    assert!(amf
        .receiver
        .recv()
        .await
        .expect("Failed mock recv")
        .is_none());

    test::ng_setup::handle(amf, logger).await;

    test_context.terminate().await;
}
