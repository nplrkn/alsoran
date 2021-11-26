

struct GnbNgapTransport {}


trait TnlaEventHandler {
    async fn tnlaEstablished();
    async fn tnlaTerminated();
}

impl GnbNgapTransport {
    async fn maintainSctpConnectionToAmf<H: TnlaEventHandler>(amfAddress: u32) {
        unimplemented();
    }
}