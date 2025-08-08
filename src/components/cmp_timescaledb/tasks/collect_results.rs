use tokio::{sync::mpsc, task::JoinHandle};
use tracing::warn;

use super::{Error, Result};

pub struct CollectResults {
    pub input: mpsc::Receiver<JoinHandle<Result<()>>>,
}

impl CollectResults {
    pub async fn spawn(mut self) -> Result<()> {
        while let Some(msg) = self.input.recv().await {
            let res = msg.await;
            if let Err(err) = res {
                warn!("Error sending to database: {}", err);
            }
        }
        Err(Error::TaskSendToDatabase)
    }
}
