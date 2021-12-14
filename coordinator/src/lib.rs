mod server;
use async_std::task::JoinHandle;
use slog::{info, Logger};
use stop_token::StopSource;

pub fn spawn(logger: Logger) -> (StopSource, JoinHandle<()>) {
    info!(logger, "Start");
    let stop_source = StopSource::new();
    let stop_token = stop_source.token();
    let task = async_std::task::spawn(async move {
        server::create("127.0.0.1:23156", stop_token).await;
        info!(logger, "Exit");
    });
    (stop_source, task)
}
