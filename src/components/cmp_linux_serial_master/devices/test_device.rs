use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc};

use crate::components_config::uart_general::RequestResponseBound;
use crate::message::{Message, MsgDataBound};

use super::super::UartMessageRaw;
use super::device_base::{ConfigPeriodicRequest, DeviceBase};

#[derive(Clone, Debug)]
pub struct TestDevice<TMsg> {
    /// Адрес
    pub address: u8,

    /// Счетчик из ESP
    pub fn_esp_counter: fn(u32) -> Message<TMsg>,
}

impl<TMsg> TestDevice<TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    /// Запустить коммуникацию с устройством
    pub async fn spawn(
        self,
        ch_tx_device_to_uart: mpsc::Sender<UartMessageRaw>,
        ch_rx_uart_to_device: broadcast::Receiver<UartMessageRaw>,
        ch_cmp_output: mpsc::Sender<Message<TMsg>>,
    ) -> super::Result<()> {
        let device: DeviceBase<TMsg, _, _> = DeviceBase {
            address: self.address,
            periodic_requests: vec![ConfigPeriodicRequest {
                period: Duration::from_millis(1000),
                request: Request::GetCounterFromEsp,
            }],
            input_request: vec![],
            fn_output: |response: Response| match response {
                Response::CounterFromEsp(data) => {
                    // let msg = self.fn_esp_counter;
                    vec![]
                }
                Response::Response2 => todo!(),
            },
        };
        device
            .spawn(ch_tx_device_to_uart, ch_rx_uart_to_device, ch_cmp_output)
            .await;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Request {
    GetCounterFromEsp,
    Request2,
}

impl RequestResponseBound for Request {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Response {
    CounterFromEsp(u32),
    Response2,
}

impl RequestResponseBound for Response {}
