// main - acts as an AMF for testing with O-RAN O-DU

use anyhow::Result;
use mocks::MockAmf;
use slog::info;

#[async_std::main]
async fn main() -> Result<()> {
    let logger = common::logging::init();
    let mut amf = MockAmf::new(&logger).await;
    amf.wait_forever();

    // Wait for connection and do NG Setup.
    amf.add_endpoint("127.0.0.1").await?;
    amf.expect_connection_established().await;
    amf.handle_ng_setup().await?;

    info!(&logger, ">> InitialUEMessage, Registration request");
    let ue = amf.receive_initial_ue_message(1).await?;

    info!(&logger, "<< DownlinkNasTransport, Authentication Request");
    let nas = vec![34];
    amf.send_downlink_nas_transport(&ue, nas).await?;

    // info!(&logger, ">> NAS Authentication response");
    // info!(&logger, "<< NAS Security mode command");
    // info!(&logger, ">> NAS Security mode complete");
    // info!(&logger, "<< NAS Registration Accept");
    // info!(&logger, ">> NAS Registration Complete");

    amf.terminate().await;

    Ok(())
}
