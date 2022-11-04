use std::{collections::HashMap, time::Instant};

use crate::config::ConnectionControlConfig;
use anyhow::Result;
use async_channel::Receiver;
use async_std::task::JoinHandle;
use connection_api::{Api, Client, JoinNgapResponse, SetupNgapResponse};
use coordination_api::models::WorkerInfo;
use futures::stream::StreamExt;
use slog::{debug, error, info, warn, Logger};
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
        mut worker_info: WorkerInfo,
        logger: &Logger,
    ) -> Result<()> {
        let worker_id = worker_info.worker_unique_id;

        // Has a long enough period elapsed that we have heard from all workers?
        if (!self.config.fast_start)
            && self.start_time.elapsed().as_secs()
                < (self.config.worker_refresh_interval_secs * 2) as u64
        {
            // No - just store the information.
            info!(
                logger,
                "Startup learning phase still in progress (uptime = {} secs)",
                self.start_time.elapsed().as_secs()
            );
            let _maybe_old_item = self.worker_info.insert(worker_id, worker_info);

            return Ok(());
        }

        let context: ClientContext = swagger::make_context!(
            ContextBuilder,
            EmptyContext,
            None as Option<AuthData>,
            XSpanIdString::default()
        );

        // Delete the old item so that it doesn't interfere with our calculations that follow.
        let _maybe_old_item = self.worker_info.remove(&worker_id);

        // Does this worker have the NGAP interface up?
        if worker_info.connected_amfs.is_empty() {
            // No, so set it up.

            // Does _any_ worker have the NGAP interface up?
            if let Some(x) = self
                .worker_info
                .values()
                .map(|x| &x.connected_amfs)
                .find(|x| !x.is_empty())
            {
                // Yes.  Join the existing NGAP instance.
                let amf_name = x.first().unwrap();
                self.join_ngap(&mut worker_info, amf_name, &context, logger)
                    .await?;
            } else {
                // No.  Set up a new NGAP instance.
                info!(logger, "Tell {:x} to set up NGAP interface", worker_id);

                self.setup_ngap(&mut worker_info, &context, logger).await?;
            }
        }

        // If this worker is not connected to the UP, try to set that up.
        if worker_info.connected_ups.is_empty() {
            // Find a worker that is connected.
            if let Some(connected_worker) = self
                .worker_info
                .values()
                .find(|x| !x.connected_ups.is_empty())
            {
                // Tell it to add this worker.
                self.add_e1ap(&connected_worker, worker_info.e1_address, logger)
                    .await;
            } else {
                debug!(logger, "No worker has a E1AP connection yet")
            }
        }

        // Same routine for the F1.
        if worker_info.connected_dus.is_empty() {
            // Find a worker that is connected.
            if let Some(connected_worker) = self
                .worker_info
                .values()
                .find(|x| !x.connected_dus.is_empty())
            {
                // Tell it to add this worker.
                self.add_f1ap(&connected_worker, worker_info.f1_address, logger)
                    .await;
            } else {
                debug!(logger, "No worker has a F1AP connection yet")
            }
        }

        // Store the worker info.
        let _ = self.worker_info.insert(worker_id, worker_info);

        // TODO get TNLA ID from message

        // Call the worker to initialize the interface

        Ok(())
    }

    fn provider(&self, base_path: &String) -> Result<Box<dyn Api<ClientContext>>> {
        Ok(if let Some(ref provider) = self.local_api_provider {
            Box::new(*provider)
        } else {
            Box::new(Client::try_new_http(base_path)?)
        })
    }

    async fn setup_ngap(
        &self,
        worker_info: &mut WorkerInfo,
        context: &ClientContext,
        logger: &Logger,
    ) -> Result<()> {
        match self
            .provider(&worker_info.connection_api_url)?
            .setup_ngap(self.config.amf_address.clone(), &context)
            .await
        {
            Ok(SetupNgapResponse::Success(amf_info)) => {
                info!(logger, "Setup NGAP ok");
                // Update the worker info to record that we now have a connected AMF.
                worker_info.connected_amfs = vec![amf_info.amf_name]
            }
            Ok(r) => error!(logger, "NGAP setup unsuccessful response - {:?}", r),
            Err(e) => error!(logger, "NGAP setup error - {}", e),
        }
        Ok(())
    }

    async fn join_ngap(
        &self,
        worker_info: &mut WorkerInfo,
        amf_name: &String,
        context: &ClientContext,
        logger: &Logger,
    ) -> Result<()> {
        let worker_id = worker_info.worker_unique_id;
        info!(logger, "Tell {:x} to join NGAP interface", worker_id);
        match self
            .provider(&worker_info.connection_api_url)?
            .join_ngap(self.config.amf_address.clone(), context)
            .await
        {
            Ok(JoinNgapResponse::Success) => {
                info!(logger, "Join NGAP ok");
                // Update the worker info to record that we now have a connected AMF.
                worker_info.connected_amfs = vec![amf_name.clone()]
            }
            Ok(r) => error!(logger, "NGAP join failure - {:?}", r),
            Err(e) => error!(logger, "NGAP join error - {}", e),
        }
        Ok(())
    }
}
