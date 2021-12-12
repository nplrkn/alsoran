use slog::o;

#[async_std::test]
async fn test_add() {
    let logger = common::logging::test_init();
    worker::run(logger.new(o!("nodetype"=> "worker"))).await;
    coordinator::run(logger.new(o!("nodetype" => "coordinator"))).await;
}
