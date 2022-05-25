use super::{Gnbcu, UeContext};

#[derive(Clone)]
pub struct RrcHandler(Gnbcu);

impl RrcHandler {
    pub fn new(gnbcu: Gnbcu) -> RrcHandler {
        RrcHandler(gnbcu)
    }

    pub fn dispatch(&self, _ue: UeContext, _message: &[u8]) {
        unimplemented!()
    }
}


