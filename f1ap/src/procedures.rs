// use crate::pdu::*;
// use crate::top_pdu::*;
// use anyhow::{anyhow, Result};
// use async_trait::async_trait;
// use net::AperCodec;
// use net::AperCodecError;
// use net::Application;
// use net::EventHandler;
// use net::InterfaceProvider;
// use net::Procedure;
// use net::RequestError;
// use net::RequestProvider;
// use net::TnlaEvent;
// use slog::Logger;

// // Autogen this
// // impl From<F1SetupRequest> for F1apPdu {
// //     fn from(x: F1SetupRequest) -> Self {
// //         F1apPdu::InitiatingMessage(InitiatingMessage::F1SetupRequest(x))
// //     }
// // }

// // impl From<F1SetupResponse> for SuccessfulOutcome {
// //     fn from(x: F1SetupResponse) -> Self {
// //         SuccessfulOutcome::F1SetupResponse(x)
// //     }
// // }
// // impl From<F1SetupFailure> for UnsuccessfulOutcome {
// //     fn from(x: F1SetupFailure) -> Self {
// //         UnsuccessfulOutcome::F1SetupFailure(x)
// //     }
// // }

// // Autogen this
// pub struct F1SetupProcedure {}
// impl Procedure for F1SetupProcedure {
//     type TopPdu = F1apPdu;
//     type Request = F1SetupRequest;
//     type Success = F1SetupResponse;
//     type Failure = F1SetupFailure;
//     const CODE: u8 = 1;
//     fn encode_request(r: Self::Request) -> Result<Vec<u8>, AperCodecError> {
//         F1apPdu::InitiatingMessage(InitiatingMessage::F1SetupRequest(r)).into_bytes()
//     }

//     fn decode_response(bytes: &[u8]) -> Result<Self::Success, RequestError<Self::Failure>> {
//         let response_pdu = Self::TopPdu::from_bytes(bytes)?;
//         match response_pdu {
//             F1apPdu::SuccessfulOutcome(SuccessfulOutcome::F1SetupResponse(x)) => Ok(x),
//             F1apPdu::UnsuccessfulOutcome(UnsuccessfulOutcome::F1SetupFailure(x)) => {
//                 Err(RequestError::UnsuccessfulOutcome(x))
//             }
//             _ => Err(RequestError::Other("Unexpected pdu contents".to_string())),
//         }
//     }
// }

// // fn map<T, E>(r: Result<T, RequestError<E>>) -> Result<F1apPdu>
// // where
// //     UnsuccessfulOutcome: From<E>,
// //     SuccessfulOutcome: From<T>,
// // {
// //     r.map(|x| F1apPdu::SuccessfulOutcome(x.into()))
// //         .or_else(|e| match e {
// //             RequestError::UnsuccessfulOutcome(x) => Ok(F1apPdu::UnsuccessfulOutcome(x.into())),
// //             RequestError::Other(s) => Err(anyhow!(format!("{}", s))),
// //         })
// // }

// #[derive(Clone)]
// pub struct F1apCu<T>(T)
// where
//     T: RequestProvider<F1SetupProcedure> + EventHandler + Clone;

// impl<T> F1apCu<T>
// where
//     T: RequestProvider<F1SetupProcedure> + EventHandler + Clone,
// {
//     pub fn new(inner: T) -> Self {
//         F1apCu(inner)
//     }
// }

// #[async_trait]
// impl<T> EventHandler for F1apCu<T>
// where
//     T: RequestProvider<F1SetupProcedure> + EventHandler,
// {
//     async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
//         self.0.handle_event(event, tnla_id, logger).await;
//     }
// }

// #[derive(Clone)]
// pub struct F1apDu<T>(T)
// where
//     T: RequestProvider<F1SetupProcedure> + EventHandler + Clone;

// impl<T> F1apDu<T>
// where
//     T: RequestProvider<F1SetupProcedure> + EventHandler + Clone,
// {
//     pub fn new(inner: T) -> Self {
//         F1apDu(inner)
//     }
// }

// #[async_trait]
// impl<T> EventHandler for F1apDu<T>
// where
//     T: RequestProvider<F1SetupProcedure> + EventHandler,
// {
//     async fn handle_event(&self, event: TnlaEvent, tnla_id: u32, logger: &Logger) {
//         self.0.handle_event(event, tnla_id, logger).await;
//     }
// }

// impl<T> Application for F1apCu<T> where T: RequestProvider<F1SetupProcedure> + EventHandler + Clone {}
// impl<T> Application for F1apDu<T> where T: RequestProvider<F1SetupProcedure> + EventHandler + Clone {}

// #[async_trait]
// impl<T> InterfaceProvider for F1apCu<T>
// where
//     T: Send + Sync + RequestProvider<F1SetupProcedure> + EventHandler,
// {
//     type TopPdu = F1apPdu;
//     async fn route_request(&self, p: F1apPdu, logger: &Logger) -> Result<F1apPdu> {
//         match match p {
//             F1apPdu::InitiatingMessage(m) => m,
//             _ => return Err(anyhow!("Not a request!")),
//         } {
//             InitiatingMessage::F1SetupRequest(req) => {
//                 match <T as RequestProvider<F1SetupProcedure>>::request(&self.0, req, logger).await
//                 {
//                     Ok(x) => Ok(F1apPdu::SuccessfulOutcome(
//                         SuccessfulOutcome::F1SetupResponse(x),
//                     )),
//                     Err(_) => todo!(),
//                 }
//             }
//             _ => todo!(),
//         }
//     }
// }

// #[async_trait]
// impl<T> InterfaceProvider for F1apDu<T>
// where
//     T: Send + Sync + RequestProvider<F1SetupProcedure> + EventHandler,
// {
//     type TopPdu = F1apPdu;
//     async fn route_request(&self, p: F1apPdu, logger: &Logger) -> Result<F1apPdu> {
//         match match p {
//             F1apPdu::InitiatingMessage(m) => m,
//             _ => return Err(anyhow!("Not a request!")),
//         } {
//             InitiatingMessage::F1SetupRequest(req) => {
//                 match <T as RequestProvider<F1SetupProcedure>>::request(&self.0, req, logger).await
//                 {
//                     Ok(x) => Ok(F1apPdu::SuccessfulOutcome(
//                         SuccessfulOutcome::F1SetupResponse(x),
//                     )),
//                     Err(_) => todo!(),
//                 }
//             }
//             _ => todo!(),
//         }
//     }
// }
