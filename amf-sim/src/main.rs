// main - acts as an AMF for testing with O-RAN O-DU

use anyhow::Result;
use mocks::MockAmf;
use slog::info;

// The purpose of this code is to cause Alsoran GNB-CU to produce the same message sequence as the O-DU CU_STUB.
//
// That is:
//    F1AP DL RRC Message Transfer + RRC SETUP COMPLETE,
//    F1AP DL RRC Message Transfer + RRC DL Information Transfer + NAS_AUTHENTICATION_RSP,
//    F1AP DL RRC Message Transfer + RRC DL Information Transfer + NAS_SECURITY_MODE_COMPLETE,
//    F1AP DL RRC Message Transfer + RRC_SECURITY_MODE_COMPLETE,
//    F1AP Ue Context Setup Request,
//    F1AP DL RRC Message Transfer + RRC Reconfiguration,
//    F1AP Ue Context Modification Request,

#[async_std::main]
async fn main() -> Result<()> {
    let logger = common::logging::init();
    let mut amf = MockAmf::new(&logger).await;
    amf.wait_forever();

    // Wait for connection and do NG Setup.
    amf.add_endpoint("127.0.0.1").await?;
    amf.expect_connection_established().await;
    amf.handle_ng_setup().await?;

    let ue = amf.receive_initial_ue_message(1).await?;
    info!(&logger, ">> Registration request");

    info!(
        &logger,
        "<< Empty NAS standing in for Authentication request"
    );
    let nas = vec![];
    amf.send_downlink_nas_transport(&ue, nas).await?;

    let _nas = amf.receive_uplink_nas_transport(&ue).await?;
    info!(&logger, ">> Nas Authentication Response");

    info!(&logger, "<< NAS standing in for Security Mode Comamnd");
    let nas = vec![];
    amf.send_downlink_nas_transport(&ue, nas).await?;

    let _nas = amf.receive_uplink_nas_transport(&ue).await?;
    info!(&logger, ">> Nas Security Mode Complete");

    amf.send_initial_context_setup_request(&ue).await?;
    amf.receive_initial_context_setup_response(&ue).await?;

    let _nas = amf.receive_uplink_nas_transport(&ue).await?;
    info!(&logger, ">> Nas Registration complete");

    amf.terminate().await;

    Ok(())
}
