use std::time::Duration;

use tokio::sync::mpsc;

use crate::executor::sleep;

use super::{Error, InnerMessage, Result};

pub struct Periodic {
    pub output: mpsc::Sender<InnerMessage>,
    pub save_by_period: Duration,
}

impl Periodic {
    pub async fn spawn(self) -> Result<()> {
        loop {
            sleep(self.save_by_period).await;

            self.output
                .send(InnerMessage::SendByTimer)
                .await
                .map_err(|_| Error::TokioMpsc {
                    task_name: "Periodic",
                })?;
        }
    }
}
