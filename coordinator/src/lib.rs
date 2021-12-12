mod server;
//use async_compat::Compat;
use slog::Logger;

pub async fn run(_logger: Logger) {
    server::create("127.0.0.1:23156").await
    //Compat::new(async { server::create("127.0.0.1:23156").await }).await
}
