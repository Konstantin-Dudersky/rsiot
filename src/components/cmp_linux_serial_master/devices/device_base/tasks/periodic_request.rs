use std::fmt::Debug;
use std::time::Duration;

use serde::{de::DeserializeOwned, Serialize};
use tokio::{sync::mpsc, time::sleep};
use tracing::info;

use crate::components_config::uart_general::UartMessage;

use super::super::super::super::UartMessageRaw;

pub struct PeriodicRequest<TRequest> {
    pub address: u8,
    pub period: Duration,
    pub request: TRequest,
    pub ch_tx_device_to_uart: mpsc::Sender<UartMessageRaw>,
}

impl<TRequest> PeriodicRequest<TRequest>
where
    TRequest: Clone + Debug + DeserializeOwned + Send + Sync + Serialize,
{
    pub async fn spawn(self) {
        loop {
            let request = UartMessage {
                address: self.address,
                payload: self.request.clone(),
            };
            info!("Request: {:?}", request);

            let uart_message_raw = request.serialize().unwrap();

            self.ch_tx_device_to_uart
                .send(uart_message_raw)
                .await
                .unwrap();

            sleep(self.period).await
        }
    }
}
