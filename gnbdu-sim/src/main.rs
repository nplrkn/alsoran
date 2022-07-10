mod ue;
use anyhow::Result;
use mocks::MockDu;
use slog::info;
use ue::Ue;

#[async_std::main]
async fn main() -> Result<()> {
    let logger = common::logging::init_terminal_logging();
    let (du, stop_source) = MockDu::new(&logger).await;

    du.establish_connection("127.0.0.1:38472".to_string())
        .await?;

    du.perform_f1_setup().await?;

    let mut ue = Ue::new();

    info!(&logger, "> RRC Setup (NAS Registration Request >");
    let nas_message = ue.recv_nas();
    du.perform_rrc_setup(nas_message).await?;

    info!(&logger, "< NAS Authentication request <");
    let nas_authentication_request = du.receive_nas().await?;
    ue.send_nas(nas_authentication_request, &logger);

    info!(&logger, "> NAS Authentication response >");
    let nas_message = ue.recv_nas();
    du.send_nas(nas_message).await?;

    info!(&logger, "< NAS Security mode command <");
    let nas_security_mode_command = du.receive_nas().await?;
    ue.send_nas(nas_security_mode_command, &logger);

    info!(&logger, "> NAS Security mode complete >");
    let nas_message = ue.recv_nas();
    du.send_nas(nas_message).await?;

    info!(&logger, "< UE ctxt setup req (Security mode command) <");
    let security_mode_command = du.receive_ue_context_setup_request().await?;

    info!(&logger, "> UE ctxt setup resp >");
    du.send_ue_context_setup_response().await?;

    info!(&logger, "> Security mode complete >");
    du.send_security_mode_complete(&security_mode_command)
        .await?;

    info!(&logger, "< Rrc Reconfiguration (Registration Accept) <");
    let nas_registration_accept = du.receive_rrc_reconfiguration().await?;
    ue.send_nas(nas_registration_accept, &logger);

    info!(&logger, "> Rrc Reconfiguration Complete >");
    du.send_rrc_reconfiguration_complete().await?;

    let nas_message = ue.recv_nas();
    info!(&logger, "> NAS Registration Complete >");
    du.send_nas(nas_message).await?;

    drop(stop_source);

    Ok(())
}
