use tokio::sync::{broadcast, mpsc};
use tracing::trace;

use crate::message::{Message, MsgDataBound};

use super::{Buffer, RequestResponseBound};

pub struct Response<TMsg, TResponse, TBuffer> {
    pub address: u8,
    pub buffer: Buffer<TBuffer>,
    pub ch_rx_fieldbus_to_device: broadcast::Receiver<TResponse>,
    pub ch_tx_output_to_filter: mpsc::Sender<Message<TMsg>>,
    pub fn_response_to_buffer: fn(TResponse, &mut TBuffer),
    pub fn_buffer_to_msgs: fn(&TBuffer) -> Vec<Message<TMsg>>,
}

impl<TMsg, TResponse, TBuffer> Response<TMsg, TResponse, TBuffer>
where
    TResponse: RequestResponseBound,
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(response) = self.ch_rx_fieldbus_to_device.recv().await {
            // Если ответ пришел для устройства с другим адресом, отбрасываем
            if self.address != response.address() {
                continue;
            }

            trace!("Response: {:?}", response);

            let mut buffer = self.buffer.lock().await;
            (self.fn_response_to_buffer)(response, &mut buffer);

            let msgs = (self.fn_buffer_to_msgs)(&buffer);
            drop(buffer);

            for msg in msgs {
                self.ch_tx_output_to_filter.send(msg).await.unwrap();
            }
        }

        Ok(())
    }
}
