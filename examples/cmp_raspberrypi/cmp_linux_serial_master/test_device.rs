//! Пример обмена данными с тестовым устройством. Реализацию см. `rsiot-examples`.

use std::time::Duration;

use async_trait::async_trait;
use rsiot::components_config::uart_general::protocol::{Protocol, UartPacket};
use rsiot::serde_utils::SerdeAlgKind;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc};

use rsiot::components_config::uart_general::{FieldbusRequest, FieldbusResponse};
use rsiot::message::{Message, MsgDataBound};

use rsiot::components_config::master_device::{
    self, BufferBound, ConfigPeriodicRequest, DeviceBase, DeviceTrait,
};

/// Тестовое устройство
#[derive(Clone, Debug)]
pub struct TestDevice<TMsg> {
    pub address: u8,

    /// Преобразование входящих сообщений в данные для сохранения в буфере
    pub fn_input: fn(&Message<TMsg>, &mut Buffer),

    /// Преобразование данных из буфера в исходящие сообщения
    pub fn_output: fn(&Buffer) -> Vec<Message<TMsg>>,
}

#[async_trait]
impl<TMsg> DeviceTrait<TMsg, FieldbusRequest, FieldbusResponse> for TestDevice<TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn spawn(
        self: Box<Self>,
        ch_rx_msgbus_to_device: broadcast::Receiver<Message<TMsg>>,
        ch_tx_device_to_fieldbus: mpsc::Sender<FieldbusRequest>,
        ch_rx_fieldbus_to_device: mpsc::Receiver<FieldbusResponse>,
        ch_tx_device_to_msgbus: mpsc::Sender<Message<TMsg>>,
    ) -> master_device::Result<()> {
        let device = DeviceBase {
            fn_init_requests: || vec![],
            periodic_requests: vec![ConfigPeriodicRequest {
                period: Duration::from_millis(500),
                fn_requests: |buffer: &Buffer| {
                    let req = buffer
                        .protocol
                        .serialize_request(Request::GetCounterFromEsp)?;
                    Ok(vec![req])
                },
            }],
            fn_msgs_to_buffer: self.fn_input,
            fn_buffer_to_request: |buffer: &Buffer| {
                let req = buffer
                    .protocol
                    .serialize_request(Request::SetCounterRpi(buffer.counter_rpi))?;
                Ok(vec![req])
            },
            fn_response_to_buffer: |response: FieldbusResponse, buffer: &mut Buffer| {
                let packet: UartPacket<Response> =
                    buffer.protocol.deserialize_response(response)?;
                match packet.data {
                    Response::CounterFromEsp(val) => buffer.counter_esp = val,
                    Response::Ok => (),
                }
                Ok(())
            },
            fn_buffer_to_msgs: self.fn_output,
            buffer_default: Buffer {
                protocol: Protocol::new(self.address, SerdeAlgKind::Postcard),
                ..Default::default()
            },
        };
        device
            .spawn(
                ch_rx_msgbus_to_device,
                ch_tx_device_to_fieldbus,
                ch_rx_fieldbus_to_device,
                ch_tx_device_to_msgbus,
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

/// Ответы
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Response {
    /// Счетчик из ESP32
    CounterFromEsp(u32),
    /// Ok
    Ok,
}

/// Буфер данных
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Buffer {
    pub protocol: Protocol,
    /// Счетчик из Rpi
    pub counter_rpi: u32,
    /// Счетчик из ESP32
    pub counter_esp: u32,
}

impl BufferBound for Buffer {}
