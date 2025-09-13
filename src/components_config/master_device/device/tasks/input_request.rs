use tokio::sync::{broadcast, mpsc};

use crate::{
    executor::CheckCapacity,
    message::{Message, MsgDataBound},
};

use super::{Buffer, BufferBound, Error};

pub struct InputRequest<TMsg, TBuffer> {
    pub buffer: Buffer<TBuffer>,
    pub ch_rx_msgbus_to_device: broadcast::Receiver<Message<TMsg>>,
    pub ch_tx_buffer: mpsc::Sender<()>,
    pub fn_msgs_to_buffer: fn(&TMsg, &mut TBuffer),
}

impl<TMsg, TBuffer> InputRequest<TMsg, TBuffer>
where
    TMsg: MsgDataBound,
    TBuffer: BufferBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(msg) = self.ch_rx_msgbus_to_device.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };

            let changed = {
                let mut buffer = self.buffer.lock().await;
                // TODO - рассмотреть возможность определять изменения через Hash. Что быстреее?
                let buffer_old = buffer.clone();
                (self.fn_msgs_to_buffer)(&msg, &mut buffer);
                *buffer != buffer_old
            };

            if changed {
                self.ch_tx_buffer
                    .check_capacity(0.2, "master_device | InputRequest")
                    .send(())
                    .await
                    .map_err(|_| Error::TokioSyncMpscSend)?;
            }
        }

        Ok(())
    }
}
