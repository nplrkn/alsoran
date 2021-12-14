mod server;
use slog::{info, Logger};
use stop_token::StopSource;

pub fn spawn(logger: Logger) -> StopSource {
    info!(logger, "Start");
    let stop_source = StopSource::new();
    let stop_token = stop_source.token();
    async_std::task::spawn(async { server::create("127.0.0.1:23156", stop_token).await });
    stop_source
}
