use async_channel::Sender;
use async_std::sync::{Arc, Mutex};

mod mock_transport_provider;
mod sctp_tnla_pool;
mod sctp_transport_provider;
mod tnla_event_handler;
mod transaction_receiver;
mod transaction_sender;
mod transport_provider;

pub use mock_transport_provider::MockTransportProvider;
pub use sctp::Message;
pub use sctp_transport_provider::SctpTransportProvider;
pub use tnla_event_handler::{TnlaEvent, TnlaEventHandler};
pub use transaction_receiver::TransactionReceiver;
pub use transaction_sender::TransactionSender;
pub use transport_provider::TransportProvider;

pub type TransactionMatchFn = Box<dyn Fn(&Message) -> bool + Send + Sync>;

pub type SharedTransactions = Arc<Mutex<Box<Vec<(TransactionMatchFn, Sender<Message>)>>>>;
