// TS 38.473
// The signalling connection shall provide in sequence delivery of F1AP messages. 
// F1AP shall be notified if the signalling connection breaks.

// TS38.472
// F1-C signalling bearer, carries signalling for an F1-C interface instance between a CU and DU.
// The bearer is implemented as a set of SCTP assocations initiated by DU.

// A DU needs to
//   -  maintain assocations to 
//   -  stick UEs to a stream

// Common elements are
//   - maintaining a set of associations
//   - single SCTP association shall be employed for F1AP elementary procedures that utilize non-UE-associated signalling 
//     - single pair of stream identifiers shall 
//   -  Selection of the SCTP association by the gNB-DU and the gNB-CU is specified in TS 38.401 [7]


use async_net::addr::AsyncToSocketAddrs;

pub struct Binding {}


/// Application Protocol Bearer, in common between NGAP, F1AP, E1AP.

/// Application Protocol Bearer Listener, used for AMF for NGAP and gNB-CU for F1AP.
pub struct Server {}

impl Server {
    /// Start the server. 
    pub async fn start<A: AsyncToSocketAddrs, H: Handler>(addr: A, handler: H) -> Result<Server> {
        implemented!();
    }
}

impl Sender for Server {

}


/// Application Protocol Bearer Client, used for gNB for NGAP and gNB-DU for F1AP.
pub struct Client {}

impl Client {
    pub async fn establish_bearer<A: AsyncToSocketAddrs, H: Handler>(addr: A, handler: H) -> Result<Client> {
        unimplemented!();
    }

    /// Set the required TNLAs.  This will cause some number of new TNLAs to be established.
    /// The establish handler will be called for each one asynchronously. 
    pub async fn set_required_tnlas(&mut self, tnlas: [RequiredTnla]) {
        unimplemented!();
    }
}

impl Sender for Client {
    async fn send_non_ue_associated_message(&self, buf: &[u8]) -> Result<usize> {
        unimplemented!();
    }

    async fn send_ue_associated_message(&self, buf: &[u8], requested_ue_tnla_binding: Binding) -> Result<(usize, Binding)> {
        unimplemented!();
    }
}

struct RequiredTnla {}

trait Sender {
    /// Send a non UE associated message.
    pub async fn send_non_ue_associated_message(&self, buf: &[u8]) -> Result<usize>;

    /// Send a UE associated message.
    pub async fn send_ue_associated_message(&self, buf: &[u8], requested_ue_tnla_binding: Binding) -> Result<(usize, Binding)>;
}

trait Handler {
    /// Receive a non UE associated message.
    pub async fn non_ue_associated_message(&self, buf: &[u8]);

    /// Receive a UE associated message.
    pub async fn ue_associated_message(&self, buf: &[u8], tnla_binding: Binding);
}
