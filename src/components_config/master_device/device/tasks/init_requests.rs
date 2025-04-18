use tokio::sync::mpsc;
use tracing::trace;

use super::{Buffer, BufferBound, Error, RequestResponseBound};

pub struct InitRequest<TFieldbusRequest, TBuffer> {
    pub buffer: Buffer<TBuffer>,
    pub fn_init_requests: fn(&TBuffer) -> Vec<TFieldbusRequest>,
    pub ch_tx_device_to_fieldbus: mpsc::Sender<TFieldbusRequest>,
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
            self.ch_tx_device_to_fieldbus
                .send(request)
                .await
                .map_err(|_| Error::TokioSyncMpsc)?;
        }

        Ok(())
    }
}
