mod sctp_association;
mod sctp_bindings;

pub use sctp_association::SctpAssociation;

pub type Message = Vec<u8>;
