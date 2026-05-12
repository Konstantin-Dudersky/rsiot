use std::time::Duration;

use tokio::{sync::mpsc, time::sleep};
use tracing::{trace, warn};

use crate::executor::CheckCapacity;

use super::{Buffer, DeviceStateType, Error, RequestResponseBound};

pub struct PeriodicRequest<TRequest, TBuffer> {
    pub buffer: Buffer<TBuffer>,
    pub device_state: DeviceStateType,
    pub period: Duration,
    pub fn_request: fn(&TBuffer) -> anyhow::Result<Vec<TRequest>>,
    pub ch_tx_request: mpsc::Sender<TRequest>,
}

impl<TRequest, TBuffer> PeriodicRequest<TRequest, TBuffer>
where
    TRequest: RequestResponseBound,
{
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
            let requests = {
                let mut buffer = self.buffer.lock().await;
                (self.fn_request)(&mut buffer)
            };

            let requests = match requests {
                Ok(v) => v,
                Err(e) => {
                    warn!("Error in fn_request: {:?}", e);
                    continue;
                }
            };

            for request in requests {
                trace!("Request: {:?}", request);
                self.ch_tx_request
                    .check_capacity(0.2, "master_device | PeriodicRequest")
                    .send(request)
                    .await
                    .map_err(|_| Error::TokioSyncMpscSend)?;
            }

            sleep(self.period).await
        }
    }
}
