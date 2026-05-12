use std::time::Duration;

use tokio::{sync::mpsc, time::sleep};

use crate::executor::CheckCapacity;

use super::{DeviceStateType, Error};

pub struct BufferPeriodic {
    pub device_state: DeviceStateType,
    pub ch_tx_need_request: mpsc::Sender<()>,
    pub period: Duration,
}

impl BufferPeriodic {
    pub async fn spawn(self) -> super::Result<()> {
        // Ждём окончания инициализации
        loop {
            if self.device_state.lock().await.init_completed {
                break;
            } else {
                sleep(Duration::from_millis(2_000)).await;
            }
        }

        loop {
            sleep(self.period).await;
            self.ch_tx_need_request
                .check_capacity(0.2, "master_device | BufferPeriodic")
                .send(())
                .await
                .map_err(|_| Error::TokioSyncMpscSend)?;
        }
    }
}
