mod control;
mod server;
use async_channel;
use async_std::task::JoinHandle;
use node_control_api::server::MakeService;
use server::Server;
use slog::{error, info, Logger};
use stop_token::StopSource;
use swagger::EmptyContext;

pub fn spawn(logger: Logger) -> (StopSource, JoinHandle<()>, JoinHandle<()>) {
    info!(logger, "Coordinator instance start");
    let stop_source = StopSource::new();
    let stop_token = stop_source.token();

    let (sender, receiver) = async_channel::bounded(1);

    let addr = "127.0.0.1:23156"
        .parse()
        .expect("Failed to parse bind address");
    let server = Server::new(sender, logger.clone());
    let service = MakeService::new(server);
    //let service = MakeAllowAllAuthenticator::new(service, "cosmo");
    let service =
        node_control_api::server::context::MakeAddContext::<_, EmptyContext>::new(service);

    let control_task = control::spawn(receiver, stop_token.clone(), logger.clone());

    let server_task = async_std::task::spawn(async move {
        let server = hyper::server::Server::bind(&addr)
            .serve(service)
            .with_graceful_shutdown(stop_token);
        if let Err(e) = server.await {
            error!(logger, "Server error: {}", e);
        } else {
            info!(logger, "Server graceful shutdown");
        }
    });
    (stop_source, control_task, server_task)
}
