//! initial_context_setup - in which the secure signaling channel is established between UE and 5G core through the GNB

use super::{GnbCuUp, Workflow};
use anyhow::Result;
use e1ap::{BearerContextSetupFailure, BearerContextSetupRequest, BearerContextSetupResponse};
use net::{RequestError, ResponseAction};

impl<'a, G: GnbCuUp> Workflow<'a, G> {
    pub async fn bearer_context_setup(
        &self,
        _r: &BearerContextSetupRequest,
    ) -> Result<ResponseAction<BearerContextSetupResponse>, RequestError<BearerContextSetupFailure>>
    {
        self.log_message("BearerContextSetupRequest << ");

        // Reply to the AMF.
        self.log_message("InitialContextSetupResponse >>");
        todo!()
    }
}
