use tokio::sync::mpsc;
use tracing::{trace, warn};

use crate::message::{Message, MsgDataBound};

use super::{Buffer, RequestResponseBound};

pub struct Response<TMsg, TResponse, TBuffer> {
    pub buffer: Buffer<TBuffer>,
    pub ch_rx_fieldbus_to_device: mpsc::Receiver<TResponse>,
    pub ch_tx_output_to_filter: mpsc::Sender<Message<TMsg>>,
    pub fn_response_to_buffer: fn(TResponse, &mut TBuffer) -> anyhow::Result<()>,
    pub fn_buffer_to_msgs: fn(&TBuffer) -> Vec<Message<TMsg>>,
}

impl<TMsg, TResponse, TBuffer> Response<TMsg, TResponse, TBuffer>
where
    TResponse: RequestResponseBound,
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(response) = self.ch_rx_fieldbus_to_device.recv().await {
            trace!("Response: {:?}", response);

            let mut buffer = self.buffer.lock().await;
            let result = (self.fn_response_to_buffer)(response, &mut buffer);

            if let Err(err) = result {
                warn!("Error in fn_response_to_buffer: {:?}", err);
            }

            let msgs = (self.fn_buffer_to_msgs)(&buffer);
            drop(buffer);

            for msg in msgs {
                self.ch_tx_output_to_filter.send(msg).await.unwrap();
            }
        }

        Ok(())
    }
}
