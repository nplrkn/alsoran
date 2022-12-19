// main - sends in the necessary stimuli to drive UE registration through the GNB-CU

mod ue;
use anyhow::Result;
use mocks::MockDu;
use slog::info;
use ue::Ue;

#[async_std::main]
async fn main() -> Result<()> {
    let logger = common::logging::init_terminal_logging();
    let mut du = MockDu::new(&logger).await;

    du.perform_f1_setup(&"127.0.0.1".to_string()).await?;

    let mut ue = Ue::new(1, &du).await;

    let registration_request = ue.build_next_nas_message();
    info!(&logger, ">> NAS Registration request");
    du.perform_rrc_setup(&mut ue.du_context, registration_request)
        .await?;

    let nas_authentication_request = du.receive_nas(&ue.du_context).await?;
    info!(&logger, "<< NAS Authentication request");
    ue.handle_nas(nas_authentication_request, &logger);

    let nas_message = ue.build_next_nas_message();
    info!(&logger, ">> NAS Authentication response");
    du.send_nas(&ue.du_context, nas_message).await?;

    let nas_security_mode_command = du.receive_nas(&ue.du_context).await?;
    info!(&logger, "<< NAS Security mode command");
    ue.handle_nas(nas_security_mode_command, &logger);

    let nas_message = ue.build_next_nas_message();
    info!(&logger, ">> NAS Security mode complete");
    du.send_nas(&ue.du_context, nas_message).await?;

    let security_mode_command = du.receive_security_mode_command(&ue.du_context).await?;
    du.send_security_mode_complete(&ue.du_context, &security_mode_command)
        .await?;

    let nas_registration_accept = du.receive_nas(&ue.du_context).await?;
    info!(&logger, "<< NAS Registration Accept");
    ue.handle_nas(nas_registration_accept, &logger);

    let nas_message = ue.build_next_nas_message();
    info!(&logger, ">> NAS Registration Complete");
    du.send_nas(&ue.du_context, nas_message).await?;

    du.terminate().await;

    Ok(())
}
