use anyhow::Result;
use async_channel::Receiver;
use async_std::task::JoinHandle;
use connection_api::{Api, Client};
use coordination_api::models::WorkerInfo;
use futures::stream::StreamExt;
use slog::{error, info, warn, Logger};
use stop_token::StopToken;
use swagger::{AuthData, ContextBuilder, EmptyContext, Push, XSpanIdString};

use crate::Config;
pub fn spawn(
    receiver: Receiver<WorkerInfo>,
    config: Config,
    stop_token: StopToken,
    logger: Logger,
) -> JoinHandle<()> {
    async_std::task::spawn(control_task(receiver, config, stop_token, logger))
}

enum NgapState {
    Uninitialized,
    Initialized,
}
struct Controller {
    pub ngap_state: NgapState,
    pub config: Config,
}

type ClientContext = swagger::make_context_ty!(
    ContextBuilder,
    EmptyContext,
    Option<AuthData>,
    XSpanIdString
);

async fn control_task(
    receiver: Receiver<WorkerInfo>,
    config: Config,
    stop_token: StopToken,
    logger: Logger,
) {
    let mut messages = receiver.take_until(stop_token);
    let mut controller = Controller {
        ngap_state: NgapState::Uninitialized,
        config,
    };
    while let Some(message) = messages.next().await {
        if let Err(e) = controller.process_worker_info(message, &logger).await {
            warn!(logger, "Error handling worker refresh - {}", e);
        }
    }
}

impl Controller {
    async fn process_worker_info(
        &mut self,
        worker_info: WorkerInfo,
        logger: &Logger,
    ) -> Result<()> {
        // If the connection list is empty, do nothing.
        // TODO: update the GNB-DU to remove a worker TNLA endpoint?
        // if message.connected_amfs.is_empty() {
        //     trace!(logger, "No connections to AMF - nothing to do");
        //     return;
        // }

        // If we have not yet initialized the NG interface to a given AMF
        // then do so.
        // TODO - base this off information communicated in the message?
        // Not necessarily - if coordinator restarts and gets a message from a newly
        // started worker, it will assume that the interface needs to be
        // initialized, when it just hasn't heard from the existing worker yet.
        // We could wait long enough for everyone to refresh though.
        let _triggered_procedure = match self.ngap_state {
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

        let client = Client::try_new_http(&worker_info.connection_api_url)?;

        match client
            .setup_ngap(self.config.amf_address.clone(), &context)
            .await
        {
            Ok(_) => {
                info!(logger, "Worker confirms successful TNLA initialization");
                self.ngap_state = NgapState::Initialized
            }
            Err(e) => error!(logger, "Failed to setup NGAP - {}", e),
        }
        Ok(())
    }
}
