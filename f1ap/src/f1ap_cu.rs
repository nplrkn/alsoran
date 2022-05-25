use super::top_pdu::*;
use crate::{InitiatingMessage, F1apPdu};
use async_trait::async_trait;
use net::{Application, EventHandler, InterfaceProvider, Procedure, RequestProvider, TnlaEvent};
use slog::{error, Logger};

#[derive(Clone)]
pub struct F1apCu<T>(pub T)
where
    T: RequestProvider<F1SetupProcedure>; // TODO

#[async_trait]
impl<T> EventHandler for F1apCu<T>
where
    T: RequestProvider<F1SetupProcedure> + EventHandler,
{
    async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
        self.0.handle_event(event, tnla_id, logger).await;
    }
}

impl<T> Application for F1apCu<T> where T: RequestProvider<F1SetupProcedure> + EventHandler + Clone + RequestProvider<InitialUlRrcMessageTransferProcedure>{}

#[async_trait]
impl<T> InterfaceProvider for F1apCu<T>
where
    T: Send + Sync + RequestProvider<F1SetupProcedure> + RequestProvider<InitialUlRrcMessageTransferProcedure>,
{
    type TopPdu = F1apPdu;
    async fn route_request(&self, p: F1apPdu, logger: &Logger) -> Option<F1apPdu> {
        let initiating_message = match p {
            F1apPdu::InitiatingMessage(m) => m,
            _ => {
                error!(logger, "Not a request!");
                return None;
            }
        };
        match initiating_message {
            InitiatingMessage::F1SetupRequest(req) => {
                F1SetupProcedure::call_provider(&self.0, req, logger).await
            },
            InitiatingMessage::InitialUlRrcMessageTransfer(req) => {
                InitialUlRrcMessageTransferProcedure::call_provider(&self.0, req, logger).await
            },
            _ => todo!(),
        }
    }
}

// async fn route_request<T>(&provider: T, req: Message, logger: &Logger) -> Result<F1apPdu> {
//     match <T as RequestProvider<F1SetupProcedure>>::request(provider, req, logger).await {
//         Ok(x) => Ok(F1apPdu::SuccessfulOutcome(NgSetupRequest(x))),
//         Err(_) => todo!(),
//     }
// }
