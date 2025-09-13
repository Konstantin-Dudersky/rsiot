use std::time::Duration;

use tokio::{sync::mpsc, time::sleep};

use crate::executor::CheckCapacity;

use super::Error;

pub struct BufferPeriodic {
    pub ch_tx_buffer: mpsc::Sender<()>,
    pub period: Duration,
}

impl BufferPeriodic {
    pub async fn spawn(self) -> super::Result<()> {
        loop {
            sleep(self.period).await;
            self.ch_tx_buffer
                .check_capacity(0.2, "master_device | BufferPeriodic")
                .send(())
                .await
                .map_err(|_| Error::TokioSyncMpscSend)?;
        }
    }
}
