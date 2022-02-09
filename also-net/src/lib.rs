use async_std::sync::{Arc, Mutex};
use futures::channel::oneshot::Sender;

mod codec;
mod mock_transport_provider;
mod sctp_tnla_pool;
mod sctp_transport_provider;
mod tnla_event_handler;
mod transaction_receiver;
mod transaction_sender;
mod transport_provider;
mod wrapper;

pub use codec::{Asn1PerCodec, Codec, JsonCodec};
pub use mock_transport_provider::MockTransportProvider;
pub use sctp_transport_provider::SctpTransportProvider;
pub use tnla_event_handler::{TnlaEvent, TnlaEventHandler};
pub use transaction_receiver::TransactionReceiver;
pub use transaction_sender::TransactionSender;
pub use transport_provider::{ClientTransportProvider, ServerTransportProvider, TransportProvider};
pub use wrapper::Wrapper;

pub type TransactionMatchFn<M> = Box<dyn Fn(&M) -> bool + Send + Sync>;

pub type SharedTransactions<M> = Arc<Mutex<Box<Vec<(TransactionMatchFn<M>, Sender<M>)>>>>;
