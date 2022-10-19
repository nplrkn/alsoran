use async_channel::Receiver;
use async_std::task::JoinHandle;
use coordination_api::models::WorkerInfo;
use futures::stream::StreamExt;
use slog::{error, info, trace, Logger};
use stop_token::StopToken;
use swagger::{AuthData, ContextBuilder, EmptyContext, Push, XSpanIdString};

pub fn spawn(
    receiver: Receiver<WorkerInfo>,
    stop_token: StopToken,
    logger: Logger,
) -> JoinHandle<()> {
    async_std::task::spawn(control_task(receiver, stop_token, logger))
}

enum NgapState {
    Uninitialized,
    Initialized,
}
struct Controller {
    pub ngap_state: NgapState,
}

type ClientContext = swagger::make_context_ty!(
    ContextBuilder,
    EmptyContext,
    Option<AuthData>,
    XSpanIdString
);

async fn control_task(receiver: Receiver<WorkerInfo>, stop_token: StopToken, logger: Logger) {
    let mut messages = receiver.take_until(stop_token);
    let mut controller = Controller {
        ngap_state: NgapState::Uninitialized,
    };
    while let Some(message) = messages.next().await {
        controller.process_worker_info(message, &logger).await
    }
}

impl Controller {
    async fn process_worker_info(&mut self, message: WorkerInfo, logger: &Logger) {
        // If the connection list is empty, do nothing.
        // TODO: update the GNB-DU to remove a worker TNLA endpoint?
        if message.connected_amfs.is_empty() {
            trace!(logger, "No connections to AMF - nothing to do");
            return;
        }

        // If we have not yet initialized the NG interface to a given AMF
        // then do so.
        // TODO - base this off information communicated in the message?
        // Not necessarily - if coordinator restarts and gets a message from a newly
        // started worker, it will assume that the interface needs to be
        // initialized, when it just hasn't heard from the existing worker yet.
        let triggered_procedure = match self.ngap_state {
            NgapState::Uninitialized => "ngsetup",
            NgapState::Initialized => "ranconfigurationupdate",
        };

        // TODO get TNLA ID from message

        // Call the worker to initialize the interface
        let context: ClientContext = swagger::make_context!(
            ContextBuilder,
            EmptyContext,
            None as Option<AuthData>,
            XSpanIdString::default()
        );

        trace!(
            logger,
            "Posting to {} to trigger {}",
            message.callback_url,
            triggered_procedure
        );

        match Client::new_http()
            .trigger_interface_management(
                message.callback_url,
                InterfaceManagementReq {
                    procedure: triggered_procedure.to_string(),
                },
                &context,
            )
            .await
        {
            Ok(TriggerInterfaceManagementResponse::InterfaceManagementResponse) => {
                info!(logger, "Worker confirms successful TNLA initialization");
                self.ngap_state = NgapState::Initialized
            }
            Ok(TriggerInterfaceManagementResponse::UnexpectedError(e)) => {
                error!(logger, "Worker reports error initializing TNLA {:?}", e)
            }
            Err(e) => error!(
                logger,
                "Failed to trigger {} - {:?}", triggered_procedure, e
            ),
        }
    }
}
