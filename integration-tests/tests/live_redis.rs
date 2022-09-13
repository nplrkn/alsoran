mod test;
use std::process;

use anyhow::Result;
use async_std;
use serial_test::serial;
pub use test::*;

#[async_std::test]
#[serial]
async fn live_redis() -> Result<()> {
    // Run Redis Server on a non-standard port
    let port = 23491;
    let child = process::Command::new("redis-server")
        .arg("--port")
        .arg(port.to_string())
        .spawn()
        .expect("Couldn't run 'redis-server'");
    let shared_child = std::sync::Arc::new(std::sync::Mutex::new(child));
    let shared_child_clone = shared_child.clone();

    std::panic::set_hook(Box::new(move |_| {
        println!("Panic hook - clean up Redis");
        shared_child_clone
            .lock()
            .unwrap()
            .kill()
            .expect("Couldn't terminate Redis");
    }));

    // Run test
    let tc = TestContext::new_with_redis(Stage::Ue1Registered, port).await?;
    tc.terminate().await;

    // Terminate Redis
    shared_child
        .lock()
        .unwrap()
        .kill()
        .expect("Couldn't terminate Redis");
    Ok(())
}
