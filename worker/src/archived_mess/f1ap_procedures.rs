/// Traits for operations on the F1 reference point.
/// For a gNB-DU
/// -  the business logic implements the CuInitiatedOperations trait
/// -  the F1 transport component implements the DuInitiatedOperations trait.
/// ...and vice verse for a gNB-CU.
use async_trait::async_trait;

#[async_trait]
pub trait DuInitiatedOperations {
    // Interface management
    async fn f1_setup_request() -> Result<F1SetupResponse>;
    async fn reset() -> Result<ResetAcknowledge>;
    async fn error_indication() -> Result<()>;
    async fn gnb_du_configuration_update() -> Result<GnbDuConfigurationUpdateAcknowledge>;

    // UE Context management
    // ...
}

#[async_trait]
pub trait CuInitiatedOperations {
    // Interface management
    // ...

    // UE Context management
    // ...
    async fn ue_context_setup_request() -> Result<UeContextSetupResponse>;
}
