use tokio::sync::broadcast;
use tracing::trace;

use crate::{
    components_config::uart_general::UartMessage,
    message::{Message, MsgDataBound},
};

use super::{Buffer, BufferBound, RequestResponseBound, TaskOutput};

pub struct InputRequest<TMsg, TRequest, TBuffer> {
    pub address: u8,
    pub buffer: Buffer<TBuffer>,
    pub ch_rx_msgbus_to_device: broadcast::Receiver<Message<TMsg>>,
    pub ch_tx_device_to_fieldbus: TaskOutput<TRequest>,
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
            let mut requests = {
                let mut buffer = self.buffer.lock().await;
                let buffer_old = buffer.clone();
                (self.fn_msgs_to_buffer)(&msg, &mut buffer);
                if *buffer == buffer_old {
                    continue;
                }
                (self.fn_buffer_to_request)(&buffer)
            };

            // Задаем адрес
            for request in requests.iter_mut() {
                request.set_address(self.address);
            }

            for request in requests {
                let uart_message = UartMessage {
                    address: self.address,
                    payload: request,
                };
                trace!("Request: {:?}", uart_message);

                // let uart_message_raw = uart_message.serialize().unwrap();

                // self.ch_tx_device_to_fieldbus
                //     .send(uart_message_raw)
                //     .await
                //     .unwrap();
            }
        }

        Ok(())
    }
}
