use tokio::sync::mpsc;
use tracing::{trace, warn};

use crate::executor::CheckCapacity;

use super::{Buffer, BufferBound, Error, RequestResponseBound};

pub struct BufferToRequests<TRequest, TBuffer> {
    pub buffer: Buffer<TBuffer>,
    pub ch_rx_buffer: mpsc::Receiver<()>,
    pub ch_tx_request: mpsc::Sender<TRequest>,
    pub fn_buffer_to_request: fn(&TBuffer) -> anyhow::Result<Vec<TRequest>>,
}

impl<TRequest, TBuffer> BufferToRequests<TRequest, TBuffer>
where
    TRequest: RequestResponseBound,
    TBuffer: BufferBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while self.ch_rx_buffer.recv().await.is_some() {
            let requests = {
                let buffer = self.buffer.lock().await;
                (self.fn_buffer_to_request)(&buffer)
            };

            let requests = match requests {
                Ok(v) => v,
                Err(e) => {
                    warn!("Error in fn_buffer_to_request: {}", e);
                    continue;
                }
            };

            for request in requests {
                trace!("Request: {:?}", request);
                self.ch_tx_request
                    .check_capacity(0.2, "master_device | BufferToRequests")
                    .send(request)
                    .await
                    .map_err(|_| Error::TokioSyncMpscSend)?;
            }
        }
        Ok(())
    }
}
