//! mock - 'base class' for the mocks

use anyhow::Result;
use async_channel::{Receiver, Sender};
use async_trait::async_trait;
use net::{
    AperSerde, Binding, SctpTransportProvider, ShutdownHandle, TnlaEvent, TnlaEventHandler,
    TransportProvider,
};
use slog::{debug, Logger};
use std::fmt::Debug;

pub trait Pdu: AperSerde + 'static + Send + Sync + Clone {}

/// Base struct for building mocks
pub struct Mock<P: Pdu> {
    transport: SctpTransportProvider,
    receiver: Receiver<Option<ReceivedPdu<P>>>,
    pub logger: Logger,
    handler: Handler<P>,
    transport_tasks: Option<ShutdownHandle>,
}

pub struct ReceivedPdu<P: Pdu> {
    pub pdu: P,
    pub assoc_id: u32,
}

#[derive(Debug, Clone)]
pub struct Handler<P: Pdu>(pub Sender<Option<ReceivedPdu<P>>>);

impl<P: Pdu> Mock<P> {
    pub async fn new(logger: Logger) -> Self {
        let (sender, receiver) = async_channel::unbounded();
        let transport = SctpTransportProvider::new();

        Mock {
            transport,
            receiver,
            logger,
            handler: Handler(sender),
            transport_tasks: None,
        }
    }

    pub async fn serve(&mut self, address: String, ppid: u32) -> Result<()> {
        let transport_tasks = self
            .transport
            .clone()
            .serve(address, ppid, self.handler.clone(), self.logger.clone())
            .await?;
        self.transport_tasks = Some(transport_tasks);
        Ok(())
    }

    pub async fn connect(&mut self, address: &str, ppid: u32) {
        self.transport
            .clone()
            .connect(address, ppid, self.handler.clone(), self.logger.clone())
            .await
            .expect("Connect failed");

        // Wait for the connection to be accepted.
        debug!(self.logger, "Wait for connection to be accepted");
        self.expect_connection().await;
    }

    pub async fn terminate(self) {
        if let Some(transport_tasks) = self.transport_tasks {
            transport_tasks.graceful_shutdown().await;
        }
        self.transport.graceful_shutdown().await;
    }

    /// Wait for connection to be established or terminated.
    pub async fn expect_connection(&self) {
        debug!(self.logger, "Wait for connection from worker");
        assert!(self
            .receiver
            .recv()
            .await
            .expect("Failed mock recv")
            .is_none());
    }

    pub async fn new_ue_binding(&self, ue_id: u32) -> Binding {
        self.transport.new_ue_binding(ue_id).await.unwrap()
    }

    pub async fn send(&self, message: Vec<u8>, assoc_id: Option<u32>) {
        self.transport
            .send_message(message, assoc_id, &self.logger)
            .await
            .expect("Failed to send message");
    }

    /// Receive a Pdu, with a 0.5s timeout.
    pub async fn receive_pdu(&self) -> P {
        self.receive_pdu_with_assoc_id().await.pdu
    }

    /// Receive a Pdu, with a 0.5s timeout.
    pub async fn receive_pdu_with_assoc_id(&self) -> ReceivedPdu<P> {
        let f = self.receiver.recv();
        async_std::future::timeout(std::time::Duration::from_millis(500), f)
            .await
            .unwrap()
            .expect("Expected message")
            .expect("Expected message")
    }
}

#[async_trait]
impl<P: Pdu> TnlaEventHandler for Handler<P> {
    async fn handle_event(&self, _event: TnlaEvent, _tnla_id: u32, _logger: &Logger) {
        self.0.send(None).await.unwrap();
    }

    async fn handle_message(
        &self,
        message: Vec<u8>,
        tnla_id: u32,
        _logger: &Logger,
    ) -> Option<Vec<u8>> {
        self.0
            .send(Some(ReceivedPdu {
                pdu: P::from_bytes(&message).unwrap(),
                assoc_id: tnla_id,
            }))
            .await
            .unwrap();
        None
    }
}
