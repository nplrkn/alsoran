use anyhow::Result;
use mocks::{AmfUeContext, CuUpUeContext, DuUeContext, SecurityModeCommand};
use slog::info;

use crate::TestContext;

pub struct DetachedUe {
    ue_id: u32,
    du_ue_context: DuUeContext,
}

pub struct RegisteredUe {
    pub ue_id: u32,
    pub du_ue_context: DuUeContext,
    pub amf_ue_context: AmfUeContext,
}

type WithAmfContext = RegisteredUe;

pub struct SetupUe(WithAmfContext);
pub struct HalfRegisteredUe(WithAmfContext);

pub struct UeWithSession {
    pub ue_id: u32,
    pub du_ue_context: DuUeContext,
    pub amf_ue_context: AmfUeContext,
    pub cu_up_ue_context: CuUpUeContext,
}

impl DetachedUe {
    pub fn new(ue_id: u32, du_ue_context: DuUeContext) -> Self {
        DetachedUe {
            ue_id,
            du_ue_context,
        }
    }

    pub async fn initial_access(mut self, tc: &TestContext) -> Result<SetupUe> {
        tc.du
            .perform_rrc_setup(&mut self.du_ue_context, Vec::new())
            .await?;
        let amf_ue_context = tc.amf.receive_initial_ue_message(self.ue_id).await?;
        Ok(SetupUe(WithAmfContext {
            ue_id: self.ue_id,
            du_ue_context: self.du_ue_context,
            amf_ue_context,
        }))
    }
}

impl SetupUe {
    pub async fn initiate_registration(
        self,
        tc: &TestContext,
    ) -> Result<(HalfRegisteredUe, SecurityModeCommand)> {
        tc.amf
            .send_initial_context_setup_request(&self.0.amf_ue_context)
            .await?;
        let security_mode_command = tc.du.receive_security_mode_command(self.0.ue_id).await?;
        Ok((HalfRegisteredUe(self.0), security_mode_command))
    }
    pub fn amf_ue_context(&mut self) -> &mut AmfUeContext {
        &mut self.0.amf_ue_context
    }
}

impl HalfRegisteredUe {
    pub async fn complete_registration(
        self,
        tc: &TestContext,
        security_mode_command: &SecurityModeCommand,
    ) -> Result<RegisteredUe> {
        tc.du
            .send_security_mode_complete(&self.0.du_ue_context, security_mode_command)
            .await?;
        tc.amf
            .receive_initial_context_setup_response(&self.0.amf_ue_context)
            .await?;
        tc.du.receive_nas(self.0.ue_id).await?;
        //info!(self.logger, "Register UE {} complete", ue_id);
        Ok(self.0)
    }
}

impl RegisteredUe {
    pub async fn establish_pdu_session(self, tc: &mut TestContext) -> Result<UeWithSession> {
        let logger = &tc.logger;
        info!(logger, "Establish PDU session for UE {}", self.ue_id);
        tc.amf
            .send_pdu_session_resource_setup(&self.amf_ue_context)
            .await?;
        let cu_up_ue_context = tc.cu_up.handle_bearer_context_setup(self.ue_id).await?;
        tc.du.handle_ue_context_setup(&self.du_ue_context).await?;
        tc.cu_up
            .handle_bearer_context_modification(&cu_up_ue_context)
            .await?;
        let _nas = tc.du.receive_rrc_reconfiguration(self.ue_id).await?;
        tc.du
            .send_rrc_reconfiguration_complete(&self.du_ue_context)
            .await?;
        tc.amf
            .receive_pdu_session_resource_setup_response(&self.amf_ue_context)
            .await?;
        info!(
            logger,
            "Finished establishing PDU session for UE {}", self.ue_id
        );
        Ok(UeWithSession {
            ue_id: self.ue_id,
            amf_ue_context: self.amf_ue_context,
            du_ue_context: self.du_ue_context,
            cu_up_ue_context,
        })
    }
}
