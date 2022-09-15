// main - sends in the necessary stimuli to drive UE registration through the GNB-CU

mod ue;
use anyhow::Result;
use mocks::MockDu;
use slog::info;
use ue::Ue;

const F1AP_SCTP_PPID: u32 = 62;

#[async_std::main]
async fn main() -> Result<()> {
    let logger = common::logging::init_terminal_logging();
    let mut du = MockDu::new(&logger).await;

    du.connect("127.0.0.1:38472".to_string(), F1AP_SCTP_PPID)
        .await;
    du.perform_f1_setup().await?;

    let mut ue = Ue::new(1);

    let registration_request = ue.build_next_nas_message();
    info!(&logger, ">> NAS Registration request");
    du.perform_rrc_setup(ue.id, registration_request).await?;

    let nas_authentication_request = du.receive_nas(ue.id).await?;
    info!(&logger, "<< NAS Authentication request");
    ue.handle_nas(nas_authentication_request, &logger);

    let nas_message = ue.build_next_nas_message();
    info!(&logger, ">> NAS Authentication response");
    du.send_nas(ue.id, nas_message).await?;

    let nas_security_mode_command = du.receive_nas(ue.id).await?;
    info!(&logger, "<< NAS Security mode command");
    ue.handle_nas(nas_security_mode_command, &logger);

    let nas_message = ue.build_next_nas_message();
    info!(&logger, ">> NAS Security mode complete");
    du.send_nas(ue.id, nas_message).await?;

    let security_mode_command = du.receive_ue_context_setup_request(ue.id).await?;
    du.send_ue_context_setup_response(ue.id).await?;
    du.send_security_mode_complete(ue.id, &security_mode_command)
        .await?;

    let nas_registration_accept = du.receive_rrc_reconfiguration(ue.id).await?;
    info!(&logger, "<< NAS Registration Accept");
    ue.handle_nas(nas_registration_accept, &logger);
    du.send_rrc_reconfiguration_complete(ue.id).await?;

    let nas_message = ue.build_next_nas_message();
    info!(&logger, ">> NAS Registration Complete");
    du.send_nas(ue.id, nas_message).await?;

    du.terminate().await;

    Ok(())
}
