use crate::TestContext;
use anyhow::Result;
use async_trait::async_trait;
use mocks::{AmfUeContext, DuUeContext, NgcSession, SecurityModeCommand};
use slog::info;

// This module has a succession of struct representing the UE's progress towards registered.

/// DetchedUe - the initial state of a UE.  Call initial_access() to get a SetupUe.
pub struct DetachedUe {
    ue_id: u32,
    du_ue_context: DuUeContext,
}

#[async_trait]
pub trait RebindUe {
    async fn rebind(&mut self, tc: &TestContext, ip_addr: &str) -> Result<()>;
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

/// SetupUe - UE that has underone RRC setup.  Call initiate_registration() to get a HalfRegisteredUe.
pub struct SetupUe(WithAmfContext);
impl SetupUe {
    pub async fn initiate_registration(self, tc: &TestContext) -> Result<HalfRegisteredUe> {
        tc.amf
            .send_initial_context_setup_request(&self.0.amf_ue_context, vec![])
            .await?;
        let security_mode_command = tc
            .du
            .receive_security_mode_command(&self.0.du_ue_context)
            .await?;
        Ok(HalfRegisteredUe(self.0, security_mode_command))
    }
}

#[async_trait]
impl RebindUe for SetupUe {
    async fn rebind(&mut self, tc: &TestContext, ip_addr: &str) -> Result<()> {
        tc.amf
            .rebind(&mut self.0.amf_ue_context.binding, ip_addr)
            .await?;
        tc.du
            .rebind(&mut self.0.du_ue_context.binding, ip_addr)
            .await
    }
}

/// HalfRegisteredUe - UE that has received security mode command as part of the registration procedure.  
/// Call complete_registration() to get a RegisteredUe.
pub struct HalfRegisteredUe(WithAmfContext, SecurityModeCommand);
impl HalfRegisteredUe {
    pub async fn complete_registration(self, tc: &TestContext) -> Result<RegisteredUe> {
        tc.du
            .send_security_mode_complete(&self.0.du_ue_context, &self.1)
            .await?;
        tc.amf
            .receive_initial_context_setup_response(&self.0.amf_ue_context)
            .await?;
        tc.du.receive_nas(&self.0.du_ue_context).await?;
        Ok(self.0)
    }
}
/// RegisteredUe - registered UE.  Call establish_pdu_session() to get a UeWithSession.
pub struct RegisteredUe {
    pub ue_id: u32,
    pub du_ue_context: DuUeContext,
    pub amf_ue_context: AmfUeContext,
}
impl RegisteredUe {
    pub async fn establish_pdu_session(mut self, tc: &mut TestContext) -> Result<UeWithSession> {
        let logger = &tc.logger;
        info!(logger, "Establish PDU session for UE {}", self.ue_id);
        let gtp_teid = tc
            .amf
            .send_pdu_session_resource_setup(&self.amf_ue_context)
            .await?;
        //let cu_up_ue_context = tc.cu_up.handle_bearer_context_setup(self.ue_id).await?;
        tc.du
            .handle_ue_context_setup(&mut self.du_ue_context)
            .await?;
        // tc.cu_up
        //     .handle_bearer_context_modification(&cu_up_ue_context)
        //     .await?;
        let _nas = tc
            .du
            .receive_rrc_reconfiguration(&self.du_ue_context)
            .await?;
        tc.du
            .send_rrc_reconfiguration_complete(&self.du_ue_context)
            .await?;
        let ngc_session = tc
            .amf
            .receive_pdu_session_resource_setup_response(&self.amf_ue_context, gtp_teid)
            .await?;
        info!(
            logger,
            "Finished establishing PDU session for UE {}", self.ue_id
        );
        Ok(UeWithSession {
            ue_id: self.ue_id,
            amf_ue_context: self.amf_ue_context,
            du_ue_context: self.du_ue_context,
            ngc_session, //cu_up_ue_context,
        })
    }
}

// The RegisteredUe fields get reused in some of the other structs too.  Supply a more generic name for that case.
type WithAmfContext = RegisteredUe;

/// UeWithSession - UE with a session set up (and hence a CuUpUeContext).
pub struct UeWithSession {
    pub ue_id: u32,
    pub du_ue_context: DuUeContext,
    pub amf_ue_context: AmfUeContext,
    pub ngc_session: NgcSession, //pub cu_up_ue_context: CuUpUeContext,
}

impl UeWithSession {
    pub async fn uplink_data_packet(&self, tc: &TestContext) -> Result<()> {
        tc.du.send_data_packet(&self.du_ue_context).await?;
        tc.amf.recv_data_packet(&self.ngc_session).await
    }
    pub async fn downlink_data_packet(&self, tc: &TestContext) -> Result<()> {
        tc.amf.send_data_packet(&self.ngc_session).await?;
        tc.du.recv_data_packet(&self.du_ue_context).await
    }
    pub async fn release_pdu_session(self, tc: &TestContext) -> Result<RegisteredUe> {
        let UeWithSession {
            ue_id,
            du_ue_context,
            amf_ue_context,
            ngc_session,
        } = self;

        tc.amf
            .send_pdu_session_resource_release(&amf_ue_context, &ngc_session)
            .await?;
        tc.amf
            .receive_pdu_session_resource_release_response(&amf_ue_context)
            .await?;

        Ok(RegisteredUe {
            ue_id,
            du_ue_context,
            amf_ue_context,
        })
    }
}
