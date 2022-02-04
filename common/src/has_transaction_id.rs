// move out of common and into some xxap common library?

pub trait HasTransactionId {
    fn request_transaction_id(&self) -> u8;
    fn response_transaction_id(&self) -> Option<u8>;
}
