use tokio::sync::mpsc;
use tracing::trace;

use crate::executor::CheckCapacity;

use super::{Buffer, BufferBound, Error, RequestResponseBound};

pub struct InitRequest<TRequest, TBuffer> {
    pub buffer: Buffer<TBuffer>,
    pub fn_init_requests: fn(&TBuffer) -> Vec<TRequest>,
    pub ch_tx_request: mpsc::Sender<TRequest>,
}

impl<TFieldbusRequest, TBuffer> InitRequest<TFieldbusRequest, TBuffer>
where
    TFieldbusRequest: RequestResponseBound,
    TBuffer: BufferBound,
{
    pub async fn spawn(self) -> super::Result<()> {
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

        Ok(())
    }
}
