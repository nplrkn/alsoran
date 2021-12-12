pub mod rrc_message {

    pub struct RrcSetupRequest;
    pub struct RrcSetup;
    //pub struct RrcSetupComplete;
    //pub struct SecurityModeCommand;
    //pub struct SecurityModeComplete;
    //pub struct RrcReconfiguration;
    //pub struct RrcReconfigurationComplete;
    pub enum Container {
        RrcSetupRequest(RrcSetupRequest),
        RrcSetup(RrcSetup),
    }
}

pub mod f1ap_message {
    pub use super::rrc_message::Container as RrcMessageContainer;
    pub struct InitalUlRrcMessageTransfer(pub RrcMessageContainer);
    pub struct DlRrcMessageTransfer(pub RrcMessageContainer);
    //pub struct UlRrcMessageTransfer(pub RrcMessageContainer);
}
