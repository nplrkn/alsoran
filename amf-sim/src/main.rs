// main - acts as an AMF for testing with O-RAN O-DU

use anyhow::Result;
use mocks::MockAmf;
use slog::info;

// The purpose of this code is to cause Alsoran GNB-CU to produce the same message sequence as the O-RAN-SC O-DU Cu Stub.
// The required call flow is documented in full at `documentation/odu.md`.
#[async_std::main]
async fn main() -> Result<()> {
    let logger = common::logging::init();
    let mut amf = MockAmf::new(&logger).await;
    amf.wait_forever();

    // Wait for connection and do NG Setup.
    amf.add_endpoint("127.0.0.1").await?;
    amf.expect_connection_established().await;
    amf.handle_ng_setup().await?;

    // Receive Ngap InitialUeMessage + Nas Registration Request
    let ue = amf.receive_initial_ue_message(1).await?;
    info!(&logger, ">> Registration request");

    // Send Nas Authentication Request
    info!(
        &logger,
        "<< Empty NAS standing in for Authentication request"
    );
    let nas = vec![];
    amf.send_downlink_nas_transport(&ue, nas).await?;

    // Receive Nas Authentication Response
    let _nas = amf.receive_uplink_nas_transport(&ue).await?;
    info!(&logger, ">> Nas Authentication Response");

    // Send Nas Security Mode Command
    info!(&logger, "<< NAS standing in for Security Mode Comamnd");
    let nas = vec![];
    amf.send_downlink_nas_transport(&ue, nas).await?;

    // Receive Nas Security Mode Complete
    let _nas = amf.receive_uplink_nas_transport(&ue).await?;
    info!(&logger, ">> Nas Security Mode Complete");

    // Send Ngap InitialContextSetupRequest
    amf.send_initial_context_setup_request(&ue).await?;

    // Receive Ngap InitialContextSetupResponse
    amf.receive_initial_context_setup_response(&ue).await?;

    // Send Nas Registration Accept
    info!(&logger, "<< NAS standing in for Registration Accept");
    let nas = vec![];
    amf.send_downlink_nas_transport(&ue, nas).await?;

    // Receive Nas Registration Complete
    let _nas = amf.receive_uplink_nas_transport(&ue).await?;
    info!(&logger, ">> Nas Registration complete");

    amf.terminate().await;

    Ok(())
}
