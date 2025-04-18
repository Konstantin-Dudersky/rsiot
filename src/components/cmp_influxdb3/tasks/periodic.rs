use std::time::Duration;

use tokio::sync::mpsc;

use crate::executor::sleep;

use super::{send_to_database_message::SendToDatabaseMessage, Error, Result};

pub struct Periodic {
    pub output: mpsc::Sender<SendToDatabaseMessage>,
    pub period: Duration,
}

impl Periodic {
    pub async fn spawn(self) -> Result<()> {
        loop {
            sleep(self.period).await;

            self.output
                .send(SendToDatabaseMessage::SendByTimer)
                .await
                .map_err(|_| Error::TokioMpsc)?;
        }
    }
}
