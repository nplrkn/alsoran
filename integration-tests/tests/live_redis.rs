mod test;
use std::process;

use anyhow::Result;
use async_std;
use serial_test::serial;
pub use test::*;

#[async_std::test]
#[serial]
async fn ue_can_register_live_redis() -> Result<()> {
    // Run Redis Server on a non-standard port
    let port = 23491;
    let mut child = process::Command::new("redis-server")
        .arg("--port")
        .arg(port.to_string())
        .spawn()
        .expect("Couldn't run 'redis-server'");

    // Run test
    let tc = TestContext::new_with_redis(Stage::UeRegistered, port).await?;
    tc.terminate().await;

    // Terminate Redis
    child.kill().expect("Couldn't terminate Redis");
    Ok(())
}
