use tokio::sync::mpsc;

use crate::{
    executor::{CheckCapacity, MsgBusInput},
    message::MsgDataBound,
};

use super::{Buffer, BufferBound, DeviceStateType, Error};

pub struct InputRequest<TMsg, TBuffer>
where
    TMsg: MsgDataBound,
{
    pub buffer: Buffer<TBuffer>,
    pub device_state: DeviceStateType,
    pub ch_rx_msgbus_to_device: MsgBusInput<TMsg>,
    pub ch_tx_need_request: mpsc::Sender<()>,
    pub fn_msgs_to_buffer: fn(&TMsg, &mut TBuffer),
}

impl<TMsg, TBuffer> InputRequest<TMsg, TBuffer>
where
    TMsg: MsgDataBound,
    TBuffer: BufferBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        let mut init_complete = false;

        while let Ok(msg) = self.ch_rx_msgbus_to_device.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };

            let need_request = {
                let mut buffer = self.buffer.lock().await;
                // TODO - рассмотреть возможность определять изменения через Hash. Что быстреее?
                let buffer_old = buffer.clone();
                (self.fn_msgs_to_buffer)(&msg, &mut buffer);
                *buffer != buffer_old
            };

            // Если инициализация еще не завершена, пропускаем сообщение
            if !init_complete {
                if self.device_state.lock().await.init_completed {
                    init_complete = true;
                } else {
                    continue;
                }
            }

            if need_request {
                self.ch_tx_need_request
                    .check_capacity(0.2, "master_device | InputRequest")
                    .send(())
                    .await
                    .map_err(|_| Error::TokioSyncMpscSend)?;
            }
        }

        Ok(())
    }
}
