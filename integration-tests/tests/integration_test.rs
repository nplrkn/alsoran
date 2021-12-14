use async_std;
use slog::o;

#[async_std::test]
async fn run_everything() {
    let logger = common::logging::test_init();
    let (coord_stop_source, coord_task) = coordinator::spawn(logger.new(o!("nodetype"=> "cu-c")));
    let (worker_stop_source, worker_task) = worker::spawn(logger.new(o!("nodetype"=> "cu-w")));

    async_std::task::sleep(std::time::Duration::from_secs(5)).await;

    drop(coord_stop_source);
    drop(worker_stop_source);
    coord_task.await;
    worker_task.await;
    drop(logger);
}
