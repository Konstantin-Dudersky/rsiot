use std::time::Duration;

use tokio::sync::mpsc;

use crate::executor::sleep;

use super::{Error, InnerMessage, Result};

pub struct Periodic {
    pub output: mpsc::Sender<InnerMessage>,
    pub period: Duration,
}

impl Periodic {
    pub async fn spawn(self) -> Result<()> {
        loop {
            sleep(self.period).await;

            self.output
                .send(InnerMessage::SendByTimer)
                .await
                .map_err(|_| Error::TokioMpsc)?;
        }
    }
}
