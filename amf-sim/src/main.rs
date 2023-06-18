// main - acts as an AMF for testing with O-RAN O-DU

use anyhow::Result;
use mocks::MockAmf;
use slog::info;

// The purpose of this code is to cause Alsoran GNB-CU to produce the same message sequence as the O-RAN-SC O-DU Cu Stub.
// The required call flow is documented in full at `documentation/odu.md`.
#[async_std::main]
async fn main() -> Result<()> {
    let logger = common::logging::init();
    let mut amf = MockAmf::new("127.0.0.1", &logger).await?;
    amf.disable_receive_timeouts();

    // Wait for connection and do NG Setup.
    amf.add_endpoint("127.0.0.1").await?;
    amf.expect_connection_established().await;
    amf.handle_ng_setup().await?;

    // Receive Ngap InitialUeMessage + Nas Registration Request
    let ue = amf.receive_initial_ue_message(1).await?;
    info!(&logger, ">> Registration request");

    // Send Nas Authentication Request
    info!(&logger, "<< Nas Authentication request");
    let nas = hex::decode(
        "7e005602020000217ac1c891b8aba0b2646e9cad34f4a0192010037859caf5e58000d58e09fc227bbf19",
    )?;
    amf.send_downlink_nas_transport(&ue, nas).await?;

    // Receive Nas Authentication Response
    let _nas = amf.receive_uplink_nas_transport(&ue).await?;
    info!(&logger, ">> Nas Authentication Response");

    // Send Nas Security Mode Command
    info!(&logger, "<< Nas Security Mode Comamnd");
    let nas = hex::decode("7e03e8e277e4007e005d010204f070f070e1360102")?;
    amf.send_downlink_nas_transport(&ue, nas).await?;

    // Receive Nas Security Mode Complete
    let _nas = amf.receive_uplink_nas_transport(&ue).await?;
    info!(&logger, ">> Nas Security Mode Complete");

    // Send Ngap InitialContextSetupRequest + Nas Registration Accept
    let nas = hex::decode("7e00420101")?;
    amf.send_initial_context_setup_request(&ue, nas).await?;

    // Receive Ngap InitialContextSetupResponse
    amf.receive_initial_context_setup_response(&ue).await?;

    // Send Nas Registration Accept
    //info!(&logger, "<< Nas Registration Accept");
    //amf.send_downlink_nas_transport(&ue, nas).await?;

    // Receive Nas Registration Complete
    let _nas = amf.receive_uplink_nas_transport(&ue).await?;
    info!(&logger, ">> Nas Registration complete");

    let teid = amf.send_pdu_session_resource_setup(&ue).await?;
    amf.receive_pdu_session_resource_setup_response(&ue, teid)
        .await?;

    amf.terminate().await;

    Ok(())
}
