mod server;

pub async fn run() {
    server::create("localhost:23156").await
}
