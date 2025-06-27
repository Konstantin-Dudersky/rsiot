use tokio::sync::{broadcast, mpsc};
use tracing::{trace, warn};

use crate::message::{Message, MsgDataBound};

use super::{Buffer, BufferBound, Error, RequestResponseBound};

pub struct InputRequest<TMsg, TRequest, TBuffer> {
    pub buffer: Buffer<TBuffer>,
    pub ch_rx_msgbus_to_device: broadcast::Receiver<Message<TMsg>>,
    pub ch_tx_device_to_fieldbus: mpsc::Sender<TRequest>,
    pub fn_msgs_to_buffer: fn(&TMsg, &mut TBuffer),
    pub fn_buffer_to_request: fn(&TBuffer) -> anyhow::Result<Vec<TRequest>>,
}

impl<TMsg, TRequest, TBuffer> InputRequest<TMsg, TRequest, TBuffer>
where
    TMsg: MsgDataBound,
    TRequest: RequestResponseBound,
    TBuffer: BufferBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(msg) = self.ch_rx_msgbus_to_device.recv().await {
            let requests = {
                let mut buffer = self.buffer.lock().await;
                let buffer_old = buffer.clone();
                let Some(msg) = msg.get_custom_data() else {
                    continue;
                };
                (self.fn_msgs_to_buffer)(&msg, &mut buffer);
                if *buffer == buffer_old {
                    continue;
                }
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
                self.ch_tx_device_to_fieldbus
                    .send(request)
                    .await
                    .map_err(|_| Error::TokioSyncMpsc)?;
            }
        }

        Ok(())
    }
}
