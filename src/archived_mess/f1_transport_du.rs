/// F1 transport used by the gNB-DU
struct F1TransportDu {}

impl F1TransportDu {
    async fn new<A: AsyncToSocketAddrs, H: CuInitiatedOperations>(connect_addr: A, handler: H) -> Result<F1TransportDu> {
        unimplemented!();
    }
}

impl DuInitiatedOperations for F1TransportDu {
    async fn f1_setup_request() -> Result<F1SetupResponse> { unimplemented!() }
    async fn reset() -> Result<ResetAcknowledge>  { unimplemented!() }
    async fn error_indication() -> Result<()>  { unimplemented!() }
    async fn gnb_du_configuration_update() -> Result<GnbDuConfigurationUpdateAcknowledge>  { unimplemented!() }
}