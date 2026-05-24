use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use tokio::{sync::mpsc, time::sleep};
use tracing::{debug, trace, warn};

use crate::executor::CheckCapacity;

use super::{Buffer, BufferBound, Error, RequestResponseBound};

pub struct InitRequest<TRequest, TBuffer> {
    pub id: String,
    pub buffer: Buffer<TBuffer>,
    pub init_completed: Arc<AtomicBool>,
    pub fn_init_requests: fn(&TBuffer) -> Vec<TRequest>,
    pub ch_tx_request: mpsc::Sender<TRequest>,
}

impl<TFieldbusRequest, TBuffer> InitRequest<TFieldbusRequest, TBuffer>
where
    TFieldbusRequest: RequestResponseBound,
    TBuffer: BufferBound,
{
    pub async fn spawn(self) -> super::Result<()> {
        loop {
            debug!("Executing init requests on device {}", self.id);

            let requests = {
                let buffer = self.buffer.lock().await;
                (self.fn_init_requests)(&buffer)
            };

            for request in requests {
                trace!("Request: {:?}", request);
                self.ch_tx_request
                    .check_capacity(0.2, "master_device | InitRequest")
                    .send(request)
                    .await
                    .map_err(|_| Error::TokioSyncMpscSend)?;
            }

            sleep(Duration::from_millis(2_000)).await;

            if self.init_completed.load(Ordering::Relaxed) {
                break;
            }
            warn!("Device {} not inited", self.id);
        }

        Ok(())
    }
}
