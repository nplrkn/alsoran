mod server;
use async_std::task::JoinHandle;
use node_control_api::server::MakeService;
use server::Server;
use slog::{error, info, Logger};
use stop_token::StopSource;
use swagger::EmptyContext;

pub fn spawn(logger: Logger) -> (StopSource, JoinHandle<()>) {
    info!(logger, "Start");
    let stop_source = StopSource::new();
    let stop_token = stop_source.token();

    let addr = "127.0.0.1:23156"
        .parse()
        .expect("Failed to parse bind address");
    let server = Server::new();
    let service = MakeService::new(server);
    //let service = MakeAllowAllAuthenticator::new(service, "cosmo");
    let service =
        node_control_api::server::context::MakeAddContext::<_, EmptyContext>::new(service);

    let task = async_std::task::spawn(async move {
        let server = hyper::server::Server::bind(&addr)
            .serve(service)
            .with_graceful_shutdown(stop_token);
        if let Err(e) = server.await {
            error!(logger, "Server error: {}", e);
        } else {
            info!(logger, "Server graceful shutdown");
        }
    });
    (stop_source, task)
}
