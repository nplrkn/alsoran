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
    fn client(&self, base_url: &String) -> Result<T>;
}

struct LocalApiProvider<T>(T);

impl<T: Api<ClientContext> + Clone> ConnectionApiProvider<T> for LocalApiProvider<T> {
    fn client(&self, _base_url: &String) -> Result<T> {
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
        base_url: &String,
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
    pub worker_info: HashMap<Uuid, WorkerInfo>,
    provider: P,
    marker: PhantomData<T>,
}

pub type ClientContext = swagger::make_context_ty!(
    ContextBuilder,
    EmptyContext,
    Option<AuthData>,
    XSpanIdString
);

async fn control_task<T: Api<ClientContext>, P: ConnectionApiProvider<T>>(
    receiver: Receiver<WorkerInfo>,
    config: ConnectionControlConfig,
    stop_token: StopToken,
    provider: P,
    logger: Logger,
) {
    let mut messages = receiver.take_until(stop_token);
    let mut controller = Controller {
        config,
        worker_info: HashMap::new(),
        provider,
        start_time: Instant::now(),
        marker: PhantomData,
    };
    while let Some(message) = messages.next().await {
        if let Err(e) = controller.process_worker_info(message, &logger).await {
            warn!(logger, "Error handling worker refresh - {}", e);
        }
    }
}

impl<T: Api<ClientContext>, P: ConnectionApiProvider<T>> Controller<T, P> {
    async fn process_worker_info(
        &mut self,
        mut this_worker: WorkerInfo,
        logger: &Logger,
    ) -> Result<()> {
        let worker_id = this_worker.worker_unique_id;

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
            let _maybe_old_item = self.worker_info.insert(worker_id, this_worker);

            return Ok(());
        }

        debug!(logger, "Process worker info {:?}", this_worker);

        let context: ClientContext = swagger::make_context!(
            ContextBuilder,
            EmptyContext,
            None as Option<AuthData>,
            XSpanIdString::default()
        );

        // Delete the old item so that it doesn't interfere with our calculations that follow.
        let _maybe_old_item = self.worker_info.remove(&worker_id);

        // Does this worker have the NGAP interface up?
        if this_worker.connected_amfs.is_empty() {
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
                info!(logger, "{:x} to join existing NGAP interface", worker_id);
                self.join_ngap(&mut this_worker, amf_name, &context, logger)
                    .await?;
            } else {
                // No.  Set up a new NGAP instance.
                info!(logger, "{:x} to set up new NGAP interface", worker_id);

                self.setup_ngap(&mut this_worker, &context, logger).await?;
            }
        }

        // If this worker is not connected to the UP, try to set that up.
        // If this worker is connected to the UP, see if there are any other workers
        // that need its help in getting added.
        if this_worker.connected_ups.is_empty() {
            // Find a worker that is connected.
            if let Some(connected_worker) = self
                .worker_info
                .values()
                .find(|x| !x.connected_ups.is_empty())
            {
                // Tell it to add this worker.
                info!(logger, "{:x} to join existing E1AP interface", worker_id);
                self.add_e1ap(
                    connected_worker,
                    this_worker.e1_address.clone().into(),
                    &context,
                    logger,
                )
                .await?;
            } else {
                debug!(logger, "Waiting for the CU-UP to set up E1AP")
            }
        } else {
            // Find all workers that are not connected and attempt to add them.
            let unconnected_workers = self
                .worker_info
                .values()
                .filter(|x| x.connected_ups.is_empty());
            for unconnected_worker in unconnected_workers {
                info!(
                    logger,
                    "{:x} to join existing E1AP interface", unconnected_worker.worker_unique_id
                );
                let _ = self
                    .add_e1ap(
                        &this_worker,
                        unconnected_worker.e1_address.clone().into(),
                        &context,
                        logger,
                    )
                    .await;
                // TODO - record current time of connection attempt in the unconnected worker state
                // Or rather pass & mut into add_e1ap() and do it there?
            }
        }

        // Same routine for the F1.
        if this_worker.connected_dus.is_empty() {
            // Find a worker that is connected.
            if let Some(connected_worker) = self
                .worker_info
                .values()
                .find(|x| !x.connected_dus.is_empty())
            {
                // Tell it to add this worker.
                info!(logger, "{:x} to join existing F1AP interface", worker_id);
                self.add_f1ap(
                    &connected_worker,
                    this_worker.f1_address.clone().into(),
                    &context,
                    logger,
                )
                .await?;
            } else {
                debug!(logger, "Waiting for the DU to set up F1AP")
            }
        } else {
            // Find all workers that are not connected and attempt to add them.
            let unconnected_workers = self
                .worker_info
                .values()
                .filter(|x| x.connected_dus.is_empty());
            for unconnected_worker in unconnected_workers {
                info!(
                    logger,
                    "{:x} to join existing F1AP interface", unconnected_worker.worker_unique_id
                );
                let _ = self.add_f1ap(
                    &this_worker,
                    unconnected_worker.f1_address.clone().into(),
                    &context,
                    logger,
                );
                // TODO - record current time of connection attempt in the unconnected worker state
                // Or rather pass & mut into add_e1ap() and do it there?
            }
        }

        // Store the worker info.
        let _ = self.worker_info.insert(worker_id, this_worker);

        Ok(())
    }

    async fn setup_ngap(
        &self,
        worker_info: &mut WorkerInfo,
        context: &ClientContext,
        logger: &Logger,
    ) -> Result<()> {
        match self
            .provider
            .client(&worker_info.connection_api_url)?
            .setup_ngap(self.config.amf_address.clone().into(), &context)
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
        match self
            .provider
            .client(&worker_info.connection_api_url)?
            .join_ngap(self.config.amf_address.clone().into(), context)
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

    async fn add_e1ap(
        &self,
        worker_info: &WorkerInfo,
        address: String,
        context: &ClientContext,
        logger: &Logger,
    ) -> Result<()> {
        match self
            .provider
            .client(&worker_info.connection_api_url)?
            .add_e1ap(address.into(), context)
            .await
        {
            Ok(AddE1apResponse::Success) => {
                info!(logger, "Add E1ap ok");
            }
            Ok(r) => error!(logger, "Failure adding E1 endpoint - {:?}", r),
            Err(e) => error!(logger, "API error adding E1 endpoint - {}", e),
        }
        Ok(())
    }

    async fn add_f1ap(
        &self,
        worker_info: &WorkerInfo,
        address: String,
        context: &ClientContext,
        logger: &Logger,
    ) -> Result<()> {
        match self
            .provider
            .client(&worker_info.connection_api_url)?
            .add_f1ap(address.into(), context)
            .await
        {
            Ok(AddF1apResponse::Success) => {
                info!(logger, "Add F1ap ok");
            }
            Ok(r) => error!(logger, "Failure adding F1 endpoint - {:?}", r),
            Err(e) => error!(logger, "API error adding F1 endpoint - {}", e),
        }
        Ok(())
    }
}
