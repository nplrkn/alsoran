use crate::config::ConnectionControlConfig;
use anyhow::Result;
use async_channel::Receiver;
use async_std::task::JoinHandle;
use connection_api::{
    AddE1apResponse, AddF1apResponse, Api, Client, JoinNgapResponse, SetupNgapResponse,
};
use coordination_api::models::WorkerInfo;
use futures::stream::StreamExt;
use hyper::Body;
use slog::{debug, error, info, warn, Logger};
use std::{collections::HashMap, marker::PhantomData, time::Instant};
use stop_token::StopToken;
use swagger::{AuthData, ContextBuilder, DropContextService, EmptyContext, Push, XSpanIdString};
use uuid::Uuid;

pub fn spawn<T: Api<ClientContext> + Clone + Send + Sync + 'static>(
    receiver: Receiver<WorkerInfo>,
    config: ConnectionControlConfig,
    stop_token: StopToken,
    local_api_provider: Option<T>,
    logger: Logger,
) -> JoinHandle<()> {
    if let Some(local_api_provider) = local_api_provider {
        async_std::task::spawn(control_task(
            receiver,
            config,
            stop_token,
            LocalApiProvider(local_api_provider),
            logger,
        ))
    } else {
        async_std::task::spawn(control_task(
            receiver,
            config,
            stop_token,
            RemoteApiProvider {},
            logger,
        ))
    }
}

trait ConnectionApiProvider<T: Api<ClientContext>> {
    fn client(&self, base_url: &str) -> Result<T>;
}

struct LocalApiProvider<T>(T);

impl<T: Api<ClientContext> + Clone> ConnectionApiProvider<T> for LocalApiProvider<T> {
    fn client(&self, _base_url: &str) -> Result<T> {
        Ok(self.0.clone())
    }
}

struct RemoteApiProvider {}
impl
    ConnectionApiProvider<
        Client<
            DropContextService<
                hyper::client::Client<hyper::client::HttpConnector, Body>,
                ClientContext,
            >,
            ClientContext,
        >,
    > for RemoteApiProvider
{
    fn client(
        &self,
        base_url: &str,
    ) -> Result<
        Client<
            DropContextService<
                hyper::client::Client<hyper::client::HttpConnector, Body>,
                ClientContext,
            >,
            ClientContext,
        >,
    > {
        Ok(Client::try_new_http(base_url)?)
    }
}

struct Controller<T, P>
where
    T: Api<ClientContext>,
    P: ConnectionApiProvider<T>,
{
    pub start_time: Instant,
    pub config: ConnectionControlConfig,
    provider: P,
    marker: PhantomData<T>,
}

pub type ClientContext = swagger::make_context_ty!(
    ContextBuilder,
    EmptyContext,
    Option<AuthData>,
    XSpanIdString
);

struct WorkerState {
    info: WorkerInfo,
    last_e1_attempt: Option<Instant>,
    last_f1_attempt: Option<Instant>,
    last_ng_attempt: Option<Instant>,
}

impl WorkerState {
    fn new(info: WorkerInfo) -> Self {
        WorkerState {
            info,
            last_e1_attempt: None,
            last_f1_attempt: None,
            last_ng_attempt: None,
        }
    }
}

async fn control_task<T: Api<ClientContext>, P: ConnectionApiProvider<T>>(
    receiver: Receiver<WorkerInfo>,
    config: ConnectionControlConfig,
    stop_token: StopToken,
    provider: P,
    logger: Logger,
) {
    let mut messages = receiver.take_until(stop_token);
    let controller = Controller {
        config,
        provider,
        start_time: Instant::now(),
        marker: PhantomData,
    };
    let mut workers = HashMap::new();
    while let Some(message) = messages.next().await {
        controller
            .process_worker_info(message, &mut workers, &logger)
            .await;
    }
}

impl<T: Api<ClientContext>, P: ConnectionApiProvider<T>> Controller<T, P> {
    async fn process_worker_info(
        &self,
        info: WorkerInfo,
        workers: &mut HashMap<Uuid, WorkerState>,
        logger: &Logger,
    ) {
        let worker_id = info.worker_unique_id;

        // Has a long enough period elapsed that we have heard from all workers?
        if (!self.config.fast_start)
            && self.start_time.elapsed().as_secs()
                < (self.config.worker_refresh_interval_secs * 2) as u64
        {
            // No - just store the information.
            debug!(
                logger,
                "Startup learning phase still in progress (uptime = {} secs)",
                self.start_time.elapsed().as_secs()
            );
            let _maybe_old_item = workers.insert(worker_id, WorkerState::new(info));

            return;
        }

        debug!(logger, "Process worker info {:?}", info);

        let context: ClientContext = swagger::make_context!(
            ContextBuilder,
            EmptyContext,
            None as Option<AuthData>,
            XSpanIdString::default()
        );

        // Delete the old item so that it doesn't interfere with our calculations that follow.
        let old_item = workers.remove(&worker_id);
        let mut this_worker = if let Some(mut x) = old_item {
            x.info = info;
            x
        } else {
            WorkerState::new(info)
        };

        // Does this worker have the NGAP interface up?
        if this_worker.info.connected_amfs.is_empty() {
            // No - set up or join NGAP as appropriate
            let setup = workers.values().all(|x| x.info.connected_amfs.is_empty());
            let _ = self
                .setup_or_join_ngap(&mut this_worker, &context, logger, setup)
                .await;
        };

        // Does this worker have the E1AP interface up?
        if this_worker.info.connected_ups.is_empty() {
            // Find a worker to help it get connected.
            if let Some(connected_worker) =
                workers.values().find(|x| !x.info.connected_ups.is_empty())
            {
                let _ = self
                    .add_e1ap(&connected_worker, &mut this_worker, &context, logger)
                    .await;
            } else {
                debug!(logger, "Waiting for the CU-UP to set up E1AP")
            }
        } else {
            debug!(logger, "Connected to CU-UP");

            // Help other workers get connected.
            for (_, worker_state) in workers.iter_mut() {
                if worker_state.info.connected_ups.is_empty() {
                    let _ = self
                        .add_e1ap(&this_worker, worker_state, &context, logger)
                        .await;
                }
            }
        }

        // Same routine for the F1.
        if this_worker.info.connected_dus.is_empty() {
            // Find a worker that is connected.
            if let Some(connected_worker) =
                workers.values().find(|x| !x.info.connected_dus.is_empty())
            {
                // Tell it to add this worker.
                info!(logger, "{:x} to join existing F1AP interface", worker_id);
                let _ = self
                    .add_f1ap(&connected_worker, &mut this_worker, &context, logger)
                    .await;
            } else {
                debug!(logger, "Waiting for the DU to set up F1AP")
            }
        } else {
            debug!(logger, "Connected to DU");

            // Find all workers that are not connected and attempt to add them.
            for (_, worker_state) in workers.iter_mut() {
                if worker_state.info.connected_dus.is_empty() {
                    let _ = self
                        .add_f1ap(&this_worker, worker_state, &context, logger)
                        .await;
                }
            }
        }

        // Store the worker info.
        let _ = workers.insert(worker_id, this_worker);
    }

    fn recently_attempted(&self, when: Option<Instant>) -> bool {
        when.map(|x| x.elapsed().as_secs() < self.config.worker_refresh_interval_secs as u64)
            .unwrap_or_default()
    }

    async fn setup_or_join_ngap(
        &self,
        worker: &mut WorkerState,
        context: &ClientContext,
        logger: &Logger,
        setup: bool,
    ) -> Result<()> {
        let id = worker.info.worker_unique_id;
        if self.recently_attempted(worker.last_ng_attempt) {
            info!(logger, "Recently tried to set up NGAP for {:x} - wait", id);
            return Ok(());
        }

        info!(
            logger,
            "{:x} to {} existing E1AP interface",
            id,
            if setup { "setup" } else { "join" }
        );
        worker.last_ng_attempt = Some(Instant::now());

        let client = self.provider.client(&worker.info.connection_api_url)?;
        let amf_address = self.config.amf_address.clone().into();

        if setup {
            match client.setup_ngap(amf_address, context).await {
                Ok(SetupNgapResponse::Success(_)) => {
                    debug!(logger, "Setup NGAP ok");
                }
                Ok(r) => error!(logger, "NGAP setup failure - {:?}", r),
                Err(e) => error!(logger, "NGAP setup error - {}", e),
            }
        } else {
            match client.join_ngap(amf_address, context).await {
                Ok(JoinNgapResponse::Success) => {
                    debug!(logger, "Join NGAP ok");
                }
                Ok(r) => error!(logger, "NGAP join failure - {:?}", r),
                Err(e) => error!(logger, "NGAP join error - {}", e),
            }
        }
        Ok(())
    }

    async fn add_e1ap(
        &self,
        helper: &WorkerState,
        new_worker: &mut WorkerState,
        context: &ClientContext,
        logger: &Logger,
    ) -> Result<()> {
        let id = new_worker.info.worker_unique_id;
        if self.recently_attempted(new_worker.last_e1_attempt) {
            info!(logger, "Recently tried to set up E1AP for {:x} - wait", id);
        } else {
            new_worker.last_e1_attempt = Some(Instant::now());
        }

        info!(logger, "{:x} to join existing E1AP interface", id);

        match self
            .provider
            .client(&helper.info.connection_api_url)?
            .add_e1ap(new_worker.info.e1_address.clone().into(), context)
            .await
        {
            Ok(AddE1apResponse::Success) => {
                debug!(logger, "Add E1ap ok");
            }
            Ok(r) => warn!(
                logger,
                "Failure adding E1AP interface for {:x} - {:?}", id, r
            ),
            Err(e) => warn!(
                logger,
                "API error adding E1AP interface for {:x} - {}", id, e
            ),
        }
        Ok(())
    }

    async fn add_f1ap(
        &self,
        helper: &WorkerState,
        new_worker: &mut WorkerState,
        context: &ClientContext,
        logger: &Logger,
    ) -> Result<()> {
        let id = new_worker.info.worker_unique_id;
        if self.recently_attempted(new_worker.last_f1_attempt) {
            info!(logger, "Recently tried to set up F1AP for {:x} - wait", id);
        } else {
            new_worker.last_f1_attempt = Some(Instant::now());
        }

        info!(logger, "{:x} to join existing F1AP interface", id);

        match self
            .provider
            .client(&helper.info.connection_api_url)?
            .add_f1ap(new_worker.info.f1_address.clone().into(), context)
            .await
        {
            Ok(AddF1apResponse::Success) => {
                debug!(logger, "Add F1ap ok");
            }
            Ok(r) => warn!(
                logger,
                "Failure adding F1AP interface for {:x} - {:?}", id, r
            ),
            Err(e) => warn!(
                logger,
                "API error adding F1AP interface for {:x} - {}", id, e
            ),
        }
        Ok(())
    }
}
