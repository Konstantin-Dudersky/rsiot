use tokio::sync::{broadcast, mpsc};
use tracing::trace;

use crate::{
    components_config::uart_general::RequestResponseBound,
    message::{Message, MsgDataBound},
};

use super::{Buffer, UartMessage, UartMessageRaw};

pub struct Response<TMsg, TResponse, TBuffer, const MESSAGE_LEN: usize> {
    pub address: u8,
    pub buffer: Buffer<TBuffer>,
    pub ch_rx_uart_to_device: broadcast::Receiver<UartMessageRaw<MESSAGE_LEN>>,
    pub ch_rx_output_to_filter: mpsc::Sender<Message<TMsg>>,
    pub fn_response_to_buffer: fn(TResponse, &mut TBuffer),
    pub fn_buffer_to_msgs: fn(&TBuffer) -> Vec<Message<TMsg>>,
}

impl<TMsg, TResponse, TBuffer, const MESSAGE_LEN: usize>
    Response<TMsg, TResponse, TBuffer, MESSAGE_LEN>
where
    TResponse: RequestResponseBound,
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(mut msg) = self.ch_rx_uart_to_device.recv().await {
            let response = UartMessage::deserialize(&mut msg);

            let response: UartMessage<TResponse> = match response {
                Ok(val) => val,
                Err(_) => {
                    continue;
                }
            };

            if self.address != response.address {
                continue;
            }

            trace!("Response: {:?}", response);

            let mut buffer = self.buffer.lock().await;
            (self.fn_response_to_buffer)(response.payload, &mut buffer);

            let msgs = (self.fn_buffer_to_msgs)(&buffer);
            drop(buffer);

            for msg in msgs {
                self.ch_rx_output_to_filter.send(msg).await.unwrap();
            }
        }

        Ok(())
    }
}
