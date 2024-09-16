use std::time::Duration;

use tokio::{sync::mpsc::Sender, time::sleep};

use crate::components::cmp_esp_spi_master::InnerMessage;

pub struct Period<TMsg> {
    pub period: Duration,
    pub output: Sender<InnerMessage<TMsg>>,
}

impl<TMsg> Period<TMsg> {
    pub async fn spawn(self) -> super::Result<()> {
        loop {
            self.output.send(InnerMessage::Periodic).await.unwrap();
            sleep(self.period).await
        }
    }
}
