use super::{Gnbcu, UeContext};
use rrc::*;
use slog::{Logger, warn};
use net::AperSerde;

#[derive(Clone)]
pub struct RrcHandler(Gnbcu);

impl RrcHandler {
    pub fn new(gnbcu: Gnbcu) -> RrcHandler {
        RrcHandler(gnbcu)
    }

    pub fn dispatch(&self, ue: UeContext, message: &[u8], logger: &Logger) {
        match match match UlCcchMessage::from_bytes(message) {
            Err(e) => { warn!(logger, "Failed to decode RRC message: {:?}", e); return }
            Ok(m) => m
        }.message {
            UlCcchMessageType::C1(m) => m
        } {
            C1_4::RrcSetupRequest(x) => self.rrc_setup_request(ue, x, logger),
            C1_4::RrcResumeRequest(_) => todo!(),
            C1_4::RrcReestablishmentRequest(_) => todo!(),
            C1_4::RrcSystemInfoRequest(_) => todo!(),  
        }
    }

    fn rrc_setup_request(&self, _ue: UeContext, _r: RrcSetupRequest, _logger: &Logger) {
        unimplemented!()
    }
}


