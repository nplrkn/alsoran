//! gnb_cu_up - the collection of services used by the GNB-CU-UP workflow business logic.
use crate::config::Config;
use async_trait::async_trait;

/// Trait representing the collection of services needed by GNB-CU-UP workflows.
#[async_trait]
pub trait GnbCuUp: Send + Sync + Clone + 'static {
    fn config(&self) -> &Config;
}
