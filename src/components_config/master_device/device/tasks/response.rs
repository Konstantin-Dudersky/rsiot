use tokio::sync::mpsc;
use tracing::{trace, warn};

use crate::message::{Message, MsgDataBound};

use super::{Buffer, Error, RequestResponseBound};

pub struct Response<TMsg, TResponse, TBuffer> {
    pub buffer: Buffer<TBuffer>,
    pub ch_rx_fieldbus_to_device: mpsc::Receiver<TResponse>,
    pub ch_tx_output_to_filter: mpsc::Sender<Message<TMsg>>,
    pub ch_tx_buffer: mpsc::Sender<()>,
    pub fn_response_to_buffer: fn(TResponse, &mut TBuffer) -> anyhow::Result<bool>,
    pub fn_buffer_to_msgs: fn(&mut TBuffer) -> Vec<TMsg>,
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

            let buffer_changed = (self.fn_response_to_buffer)(response, &mut buffer);
            let msgs = (self.fn_buffer_to_msgs)(&mut buffer);

            drop(buffer);

            for msg in msgs {
                let msg = Message::new_custom(msg);
                self.ch_tx_output_to_filter
                    .send(msg)
                    .await
                    .map_err(|_| Error::TokioSyncMpsc)?;
            }

            match buffer_changed {
                Ok(buffer_changed) => {
                    if buffer_changed {
                        self.ch_tx_buffer
                            .send(())
                            .await
                            .map_err(|_| Error::TokioSyncMpsc)?;
                    }
                }
                Err(e) => {
                    warn!("Error in fn_response_to_buffer: {:?}", e);
                }
            };
        }

        Ok(())
    }
}
