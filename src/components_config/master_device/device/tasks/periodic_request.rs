use std::time::Duration;

use tokio::{sync::mpsc, time::sleep};
use tracing::{trace, warn};

use super::{Buffer, Error, RequestResponseBound};

pub struct PeriodicRequest<TRequest, TBuffer> {
    pub buffer: Buffer<TBuffer>,
    pub period: Duration,
    pub fn_request: fn(&TBuffer) -> anyhow::Result<Vec<TRequest>>,
    pub ch_tx_device_to_fieldbus: mpsc::Sender<TRequest>,
}

impl<TRequest, TBuffer> PeriodicRequest<TRequest, TBuffer>
where
    TRequest: RequestResponseBound,
{
    pub async fn spawn(self) -> super::Result<()> {
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
                self.ch_tx_device_to_fieldbus
                    .send(request)
                    .await
                    .map_err(|_| Error::TokioSyncMpsc)?;
            }

            sleep(self.period).await
        }
    }
}
