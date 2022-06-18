use anyhow::Result;
use rrc::UlDcchMessage;

pub struct RrcResponseBinding {}

impl RrcResponseBinding {
    pub async fn recv(self) -> Result<UlDcchMessage> {
        todo!()
    }
}
