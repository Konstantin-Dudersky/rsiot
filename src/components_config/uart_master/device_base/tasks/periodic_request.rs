use std::fmt::Debug;
use std::time::Duration;

use serde::{de::DeserializeOwned, Serialize};
use tokio::{sync::mpsc, time::sleep};
use tracing::trace;

use super::{Buffer, UartMessage, UartMessageRaw};

pub struct PeriodicRequest<TRequest, TBuffer, const MESSAGE_LEN: usize> {
    pub address: u8,
    pub buffer: Buffer<TBuffer>,
    pub period: Duration,
    pub request: fn(&TBuffer) -> TRequest,
    pub ch_tx_device_to_uart: mpsc::Sender<UartMessageRaw<MESSAGE_LEN>>,
}

impl<TRequest, TBuffer, const MESSAGE_LEN: usize> PeriodicRequest<TRequest, TBuffer, MESSAGE_LEN>
where
    TRequest: Clone + Debug + DeserializeOwned + Send + Sync + Serialize,
{
    pub async fn spawn(self) -> super::Result<()> {
        loop {
            let request = {
                let mut buffer = self.buffer.lock().await;
                (self.request)(&mut buffer)
            };

            let request = UartMessage {
                address: self.address,
                payload: request,
            };
            trace!("Request: {:?}", request);

            let uart_message_raw = request.serialize().unwrap();

            self.ch_tx_device_to_uart
                .send(uart_message_raw)
                .await
                .unwrap();

            sleep(self.period).await
        }
    }
}
