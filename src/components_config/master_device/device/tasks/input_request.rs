use tokio::sync::{broadcast, mpsc};

use crate::message::{Message, MsgDataBound};

use super::{
    set_address_and_send_to_fieldbus::set_address_and_send_to_fieldbus, Buffer, BufferBound,
    RequestResponseBound,
};

pub struct InputRequest<TMsg, TRequest, TBuffer> {
    pub address: u8,
    pub buffer: Buffer<TBuffer>,
    pub ch_rx_msgbus_to_device: broadcast::Receiver<Message<TMsg>>,
    pub ch_tx_device_to_fieldbus: mpsc::Sender<TRequest>,
    pub fn_msgs_to_buffer: fn(&Message<TMsg>, &mut TBuffer),
    pub fn_buffer_to_request: fn(&TBuffer) -> Vec<TRequest>,
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
                (self.fn_msgs_to_buffer)(&msg, &mut buffer);
                if *buffer == buffer_old {
                    continue;
                }
                (self.fn_buffer_to_request)(&buffer)
            };

            set_address_and_send_to_fieldbus(
                requests,
                self.address,
                &self.ch_tx_device_to_fieldbus,
            )
            .await;
        }

        Ok(())
    }
}
