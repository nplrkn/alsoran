//! mock - 'base class' for the mocks

use async_channel::{Receiver, Sender};
use async_trait::async_trait;
use net::{
    AperSerde, SctpTransportProvider, ShutdownHandle, TnlaEvent, TnlaEventHandler,
    TransportProvider,
};
use slog::{trace, Logger};
use std::fmt::Debug;

pub trait Pdu: AperSerde + 'static + Send + Sync + Clone {}

/// Base struct for building mocks
pub struct Mock<P: Pdu> {
    transport: SctpTransportProvider,
    receiver: Receiver<Option<P>>,
    pub logger: Logger,
    handler: Option<Handler<P>>,
    transport_tasks: Option<ShutdownHandle>,
}

#[derive(Debug, Clone)]
pub struct Handler<P: Pdu>(pub Sender<Option<P>>);

impl<P: Pdu> Mock<P> {
    pub async fn new(logger: Logger) -> Self {
        let (sender, receiver) = async_channel::unbounded();
        let transport = SctpTransportProvider::new();
        let handler = Some(Handler(sender));

        Mock {
            transport,
            receiver,
            logger,
            handler,
            transport_tasks: None,
        }
    }

    pub async fn serve(&mut self, address: String, ppid: u32) {
        let transport_tasks = self
            .transport
            .clone()
            .serve(
                address,
                ppid,
                std::mem::take(&mut self.handler).unwrap(),
                self.logger.clone(),
            )
            .await
            .expect("Server bind failed");
        self.transport_tasks = Some(transport_tasks);
    }

    pub async fn connect(&mut self, address: String, ppid: u32) {
        let transport_tasks = self
            .transport
            .clone()
            .maintain_connection(
                address,
                ppid,
                std::mem::take(&mut self.handler).unwrap(),
                self.logger.clone(),
            )
            .await
            .expect("Connect failed");
        self.transport_tasks = Some(transport_tasks);

        // Wait for the connection to be accepted.
        trace!(self.logger, "Wait for connection to be accepted");
        self.expect_connection().await;
    }

    pub async fn terminate(self) {
        if let Some(transport_tasks) = self.transport_tasks {
            transport_tasks.graceful_shutdown().await;
        }
    }

    /// Wait for connection to be established or terminated.
    pub async fn expect_connection(&self) {
        trace!(self.logger, "Wait for connection from worker");
        assert!(self
            .receiver
            .recv()
            .await
            .expect("Failed mock recv")
            .is_none());
    }

    pub async fn send(&self, message: Vec<u8>) {
        self.transport
            .send_message(message, &self.logger)
            .await
            .expect("Failed to send message");
    }

    /// Receive a Pdu, with a 0.5s timeout.
    /// By default, this function asserts if there is a second message waiting after receiving the PDU.
    pub async fn receive_pdu(&self) -> P {
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
        _tnla_id: u32,
        _logger: &Logger,
    ) -> Option<Vec<u8>> {
        self.0
            .send(Some(P::from_bytes(&message).unwrap()))
            .await
            .unwrap();
        None
    }
}
