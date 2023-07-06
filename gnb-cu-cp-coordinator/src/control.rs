use crate::config::ConnectionControlConfig;
use anyhow::Result;
use async_channel::Receiver;
use async_std::task::JoinHandle;
use connection_api::{
    models::{ConnectionInfo, OperationType},
    AddConnectionResponse, Api, Client,
};
use coordination_api::models::{RefreshWorker, WorkerInfo};
use futures::stream::StreamExt;
use hyper::Body;
use slog::{debug, info, warn, Logger};
use std::{collections::HashMap, marker::PhantomData, time::Instant};
use stop_token::StopToken;
use swagger::{AuthData, ContextBuilder, DropContextService, EmptyContext, Push, XSpanIdString};
use uuid::Uuid;

pub fn spawn<T: Api<ClientContext> + Clone + Send + Sync + 'static>(
    receiver: Receiver<RefreshWorker>,
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
    e1: ConnectionState,
    f1: ConnectionState,
    ng: ConnectionState,
}

impl WorkerState {
    fn new(refresh: RefreshWorker) -> Self {
        WorkerState {
            info: refresh.worker_info,
            e1: ConnectionState::new(refresh.connection_state.e1_up),
            f1: ConnectionState::new(refresh.connection_state.f1_up),
            ng: ConnectionState::new(refresh.connection_state.ng_up),
        }
    }
}

#[derive(Default)]
struct ConnectionState {
    last_attempt: Option<Instant>,
    up: bool,
}

impl ConnectionState {
    fn new(up: bool) -> Self {
        ConnectionState {
            last_attempt: None,
            up,
        }
    }
}

async fn control_task<T: Api<ClientContext>, P: ConnectionApiProvider<T>>(
    receiver: Receiver<RefreshWorker>,
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
        refresh: RefreshWorker,
        workers: &mut HashMap<Uuid, WorkerState>,
        logger: &Logger,
    ) {
        let worker_id = refresh.worker_id;

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
            let _maybe_old_item = workers.insert(worker_id, WorkerState::new(refresh));

            return;
        }

        debug!(logger, "Process worker info {:?}", refresh);

        let context: ClientContext = swagger::make_context!(
            ContextBuilder,
            EmptyContext,
            None as Option<AuthData>,
            XSpanIdString::default()
        );

        // Delete the old item so that it doesn't interfere with our calculations that follow.
        let old_item = workers.remove(&worker_id);
        let mut this_worker = if let Some(mut x) = old_item {
            // TODO - instead, skip if revision number is out of date
            x.info = refresh.worker_info;
            x.e1.up = refresh.connection_state.e1_up;
            x.f1.up = refresh.connection_state.f1_up;
            x.ng.up = refresh.connection_state.ng_up;
            x
        } else {
            WorkerState::new(refresh)
        };

        // Does this worker have the NGAP interface up?
        if !this_worker.ng.up {
            // No - set up or join NGAP as appropriate
            let setup = !workers.values().any(|x| x.ng.up);
            let _ = self
                .setup_or_join_ngap(&worker_id, &mut this_worker, &context, logger, setup)
                .await;
        };

        // Does this worker have the E1AP interface up?
        if !this_worker.e1.up {
            // Find a worker to help it get connected.
            if let Some(connected_worker) = workers.values().find(|x| x.e1.up) {
                let _ = self
                    .add_e1ap(
                        &connected_worker,
                        &worker_id,
                        &mut this_worker,
                        &context,
                        logger,
                    )
                    .await;
            } else {
                debug!(logger, "Waiting for the CU-UP to set up E1AP")
            }
        } else {
            debug!(logger, "Connected to CU-UP");

            // Help other workers get connected.
            for (other_worker_id, other_worker_state) in workers.iter_mut() {
                if !other_worker_state.e1.up {
                    let _ = self
                        .add_e1ap(
                            &this_worker,
                            other_worker_id,
                            other_worker_state,
                            &context,
                            logger,
                        )
                        .await;
                }
            }
        }

        // Same routine for the F1.
        if !this_worker.f1.up {
            // Find a worker that is connected.
            if let Some(connected_worker) = workers.values().find(|x| x.f1.up) {
                // Tell it to add this worker.
                let _ = self
                    .add_f1ap(
                        connected_worker,
                        &worker_id,
                        &mut this_worker,
                        &context,
                        logger,
                    )
                    .await;
            } else {
                debug!(logger, "Waiting for the DU to set up F1AP")
            }
        } else {
            debug!(logger, "Connected to DU");

            // Find all workers that are not connected and attempt to add them.
            for (other_worker_id, other_worker_state) in workers.iter_mut() {
                if !other_worker_state.f1.up {
                    let _ = self
                        .add_f1ap(
                            &this_worker,
                            other_worker_id,
                            other_worker_state,
                            &context,
                            logger,
                        )
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
        worker_id: &Uuid,
        worker: &mut WorkerState,
        context: &ClientContext,
        logger: &Logger,
        setup: bool,
    ) -> Result<()> {
        debug!(
            logger,
            "{:x} to {} NG interface",
            worker_id,
            if setup { "setup" } else { "join" }
        );
        self.add_connection(
            &worker.info.connection_api_url,
            worker_id,
            &self.config.amf_address,
            &mut worker.ng,
            if setup {
                OperationType::SetupNg
            } else {
                OperationType::JoinNg
            },
            context,
            logger,
        )
        .await
    }

    async fn add_e1ap(
        &self,
        helper: &WorkerState,
        new_worker_id: &Uuid,
        new_worker: &mut WorkerState,
        context: &ClientContext,
        logger: &Logger,
    ) -> Result<()> {
        info!(logger, "{:x} to get added to E1 interface", new_worker_id);
        self.add_connection(
            &helper.info.connection_api_url,
            new_worker_id,
            &new_worker.info.e1_address,
            &mut new_worker.e1,
            OperationType::AddE1,
            context,
            logger,
        )
        .await
    }

    async fn add_f1ap(
        &self,
        helper: &WorkerState,
        new_worker_id: &Uuid,
        new_worker: &mut WorkerState,
        context: &ClientContext,
        logger: &Logger,
    ) -> Result<()> {
        info!(logger, "{:x} to get added to F1 interface", new_worker_id);
        self.add_connection(
            &helper.info.connection_api_url,
            new_worker_id,
            &new_worker.info.f1_address,
            &mut new_worker.f1,
            OperationType::AddF1,
            context,
            logger,
        )
        .await
    }

    async fn add_connection(
        &self,
        url: &str,
        added_worker_id: &Uuid,
        ip_address: &str,
        connection_state: &mut ConnectionState,
        op: OperationType,
        context: &ClientContext,
        logger: &Logger,
    ) -> Result<()> {
        let id = added_worker_id;
        if self.recently_attempted(connection_state.last_attempt) {
            debug!(logger, "Skip {} for {:x} - recently attempted", op, id);
            return Ok(());
        }

        connection_state.last_attempt = Some(Instant::now());

        match self
            .provider
            .client(url)?
            .add_connection(
                ConnectionInfo {
                    operation_type: op,
                    ip_address: ip_address.into(),
                },
                context,
            )
            .await
        {
            Ok(AddConnectionResponse::Success) => {
                debug!(logger, "Ok");

                // Update our local view of whether this connection is up.
                connection_state.up = true;

                // TODO: store the revision number so that this doesn't get overwritten by an
                // out of date refresh.
            }
            Ok(r) => debug!(logger, "Failure of {} for {:x} - {:?}", op, id, r),
            Err(e) => warn!(logger, "API error in {} for {:x} - {}", op, id, e),
        }

        Ok(())
    }
}
