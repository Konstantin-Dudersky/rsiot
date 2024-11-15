use tokio::sync::{broadcast, mpsc};
use tracing::info;

use crate::{
    components_config::uart_general::RequestResponseBound,
    message::{Message, MsgDataBound},
};

use super::super::super::super::{UartMessage, UartMessageRaw};

pub struct Output<TMsg, TResponse> {
    pub address: u8,
    pub ch_tx_uart_to_device: broadcast::Receiver<UartMessageRaw>,
    pub ch_cmp_output: mpsc::Sender<Message<TMsg>>,
    pub fn_output: fn(TResponse) -> Vec<Message<TMsg>>,
}

impl<TMsg, TResponse> Output<TMsg, TResponse>
where
    TResponse: RequestResponseBound,
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) {
        while let Ok(mut msg) = self.ch_tx_uart_to_device.recv().await {
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

            info!("Response: {:?}", response);

            let msgs = (self.fn_output)(response.payload);

            for msg in msgs {
                self.ch_cmp_output.send(msg).await.unwrap();
            }
        }
    }
}
