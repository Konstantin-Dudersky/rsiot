//! Пример обмена данными с тестовым устройством. Реализацию см. `rsiot-examples`.

use std::time::Duration;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc};

use crate::components_config::uart_general::{BufferBound, RequestResponseBound};
use crate::message::{Message, MsgDataBound};

use super::{ConfigPeriodicRequest, DeviceBase, DeviceTrait, UartMessageRaw};

/// Тестовое устройство
#[derive(Clone, Debug)]
pub struct TestDevice<TMsg> {
    /// Адрес
    pub address: u8,

    /// Преобразование входящих сообщений в данные для сохранения в буфере
    pub fn_input: fn(&Message<TMsg>, &mut Buffer),

    /// Преобразование данных из буфера в исходящие сообщения
    pub fn_output: fn(&Buffer) -> Vec<Message<TMsg>>,
}

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> DeviceTrait<TMsg> for TestDevice<TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn spawn(
        self: Box<Self>,
        ch_tx_device_to_uart: mpsc::Sender<UartMessageRaw>,
        ch_rx_uart_to_device: broadcast::Receiver<UartMessageRaw>,
        ch_tx_msgbus_to_device: broadcast::Receiver<Message<TMsg>>,
        ch_rx_device_to_msgbus: mpsc::Sender<Message<TMsg>>,
    ) -> super::Result<()> {
        let device: DeviceBase<TMsg, _, _, _> = DeviceBase {
            address: self.address,
            periodic_requests: vec![ConfigPeriodicRequest {
                period: Duration::from_millis(500),
                fn_request: |_buffer| Request::GetCounterFromEsp,
            }],
            fn_msgs_to_buffer: self.fn_input,
            fn_buffer_to_request: |buffer: &Buffer| {
                vec![Request::SetCounterRpi(buffer.counter_rpi)]
            },
            fn_response_to_buffer: |response: Response, buffer: &mut Buffer| match response {
                Response::CounterFromEsp(val) => buffer.counter_esp = val,
                Response::Ok => (),
            },
            fn_buffer_to_msgs: self.fn_output,
            buffer_default: Buffer::default(),
        };
        device
            .spawn(
                ch_tx_device_to_uart,
                ch_rx_uart_to_device,
                ch_tx_msgbus_to_device,
                ch_rx_device_to_msgbus,
            )
            .await?;
        Ok(())
    }
}

/// Запросы
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Request {
    /// Запрос значения счетчика из ESP32
    GetCounterFromEsp,
    /// Передать значение своего счетчика
    SetCounterRpi(u32),
}

impl RequestResponseBound for Request {}

/// Ответы
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Response {
    /// Счетчик из ESP32
    CounterFromEsp(u32),
    /// Ok
    Ok,
}

impl RequestResponseBound for Response {}

/// Буфер данных
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Buffer {
    /// Счетчик из Rpi
    pub counter_rpi: u32,
    /// Счетчик из ESP32
    pub counter_esp: u32,
}

impl BufferBound for Buffer {}
