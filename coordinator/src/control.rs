use std::{collections::HashMap, time::Instant};

use crate::config::ConnectionControlConfig;
use anyhow::Result;
use async_channel::Receiver;
use async_std::task::JoinHandle;
use connection_api::{Api, Client, SetupNgapResponse};
use coordination_api::models::WorkerInfo;
use futures::stream::StreamExt;
use slog::{error, info, warn, Logger};
use stop_token::StopToken;
use swagger::{AuthData, ContextBuilder, EmptyContext, Push, XSpanIdString};
use uuid::Uuid;

pub fn spawn<T: Api<ClientContext> + Clone + Send + Sync + 'static>(
    receiver: Receiver<WorkerInfo>,
    config: ConnectionControlConfig,
    stop_token: StopToken,
    local_api_provider: Option<T>,
    logger: Logger,
) -> JoinHandle<()> {
    async_std::task::spawn(control_task(
        receiver,
        config,
        stop_token,
        local_api_provider,
        logger,
    ))
}

struct Controller<T>
where
    T: Api<ClientContext>,
{
    pub start_time: Instant,
    pub config: ConnectionControlConfig,
    pub local_api_provider: Option<T>,
    pub worker_info: HashMap<Uuid, WorkerInfo>,
}

pub type ClientContext = swagger::make_context_ty!(
    ContextBuilder,
    EmptyContext,
    Option<AuthData>,
    XSpanIdString
);

async fn control_task<T: Api<ClientContext>>(
    receiver: Receiver<WorkerInfo>,
    config: ConnectionControlConfig,
    stop_token: StopToken,
    local_api_provider: Option<T>,
    logger: Logger,
) {
    let mut messages = receiver.take_until(stop_token);
    let mut controller = Controller {
        config,
        local_api_provider,
        worker_info: HashMap::new(),
        start_time: Instant::now(),
    };
    while let Some(message) = messages.next().await {
        if let Err(e) = controller.process_worker_info(message, &logger).await {
            warn!(logger, "Error handling worker refresh - {}", e);
        }
    }
}

impl<T: Api<ClientContext>> Controller<T> {
    async fn process_worker_info(
        &mut self,
        worker_info: WorkerInfo,
        logger: &Logger,
    ) -> Result<()> {
        let this_worker_key = worker_info.worker_unique_id;

        // Store the worker info, overwriting the previous item
        let _maybe_old_item = self.worker_info.insert(this_worker_key, worker_info);

        // Has a long enough period elapsed that we have heard from all workers?
        if (!self.config.fast_start)
            && self.start_time.elapsed().as_secs()
                < (self.config.worker_refresh_interval_secs * 2) as u64
        {
            info!(
                logger,
                "Startup learning phase still in progress (uptime = {} secs)",
                self.start_time.elapsed().as_secs()
            );
            return Ok(());
        }

        // Does this worker have the NGAP interface up?
        let this_worker = &self.worker_info[&this_worker_key];
        if this_worker.connected_amfs.is_empty() {
            // No, so set it up.

            // Does any worker have the NGAP interface up?
            if self
                .worker_info
                .values()
                .find(|w| !w.connected_amfs.is_empty())
                .is_none()
            {
                // No.  Set up a new NGAP instance.
                match self.setup_ngap(&this_worker).await {
                    Ok(SetupNgapResponse::SuccessfulSetup(amf_info)) => {
                        info!(logger, "Setup NGAP ok");
                        // Update the worker info to record that we now have a connected AMF.
                        match self.worker_info.get_mut(&this_worker_key) {
                            None => unreachable!(),
                            Some(this_worker_info) => {
                                this_worker_info.connected_amfs = vec![amf_info.amf_name]
                            }
                        }
                    }
                    Ok(r) => error!(logger, "NGAP setup unsuccessful response - {:?}", r),
                    Err(e) => error!(logger, "NGAP setup error - {}", e),
                }
            } else {
                unimplemented!()
            }
        }

        // TODO get TNLA ID from message

        // Call the worker to initialize the interface

        Ok(())
    }

    async fn setup_ngap(&self, worker_info: &WorkerInfo) -> Result<SetupNgapResponse> {
        let context: ClientContext = swagger::make_context!(
            ContextBuilder,
            EmptyContext,
            None as Option<AuthData>,
            XSpanIdString::default()
        );

        let response = if let Some(ref provider) = self.local_api_provider {
            provider
                .setup_ngap(self.config.amf_address.clone(), &context)
                .await?
        } else {
            let client = Client::try_new_http(&worker_info.connection_api_url)?;
            client
                .setup_ngap(self.config.amf_address.clone(), &context)
                .await?
        };
        Ok(response)
    }
}
