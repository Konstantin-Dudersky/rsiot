#![allow(clippy::module_inception)]

use std::sync::Arc;

use futures::TryFutureExt;
use tokio::{
    sync::{broadcast, mpsc, Mutex},
    task::JoinSet,
};

use crate::{
    components::shared_tasks,
    components_config::uart_general::{BufferBound, RequestResponseBound},
    message::Message,
};
use crate::{executor::join_set_spawn, message::MsgDataBound};

use super::{config::*, tasks, UartMessageRaw};

pub struct DeviceBase<TMsg, TRequest, TResponse, TBuffer> {
    pub address: u8,
    pub periodic_requests: Vec<ConfigPeriodicRequest<TRequest, TBuffer>>,
    pub fn_msgs_to_buffer: fn(&Message<TMsg>, &mut TBuffer),
    pub fn_buffer_to_request: fn(&TBuffer) -> Vec<TRequest>,
    pub fn_response_to_buffer: fn(TResponse, &mut TBuffer),
    pub fn_buffer_to_msgs: fn(&TBuffer) -> Vec<Message<TMsg>>,
    pub buffer_default: TBuffer,
}

impl<TMsg, TRequest, TResponse, TBuffer> DeviceBase<TMsg, TRequest, TResponse, TBuffer>
where
    TRequest: 'static + RequestResponseBound,
    TResponse: 'static + RequestResponseBound,
    TMsg: MsgDataBound + 'static,
    TBuffer: 'static + BufferBound,
{
    pub async fn spawn(
        self,
        ch_tx_device_to_uart: mpsc::Sender<UartMessageRaw>,
        ch_rx_uart_to_device: broadcast::Receiver<UartMessageRaw>,
        ch_rx_msgbus_to_device: broadcast::Receiver<Message<TMsg>>,
        ch_tx_device_to_msgbus: mpsc::Sender<Message<TMsg>>,
    ) -> super::Result<()> {
        let buffer = self.buffer_default;
        let buffer = Arc::new(Mutex::new(buffer));

        let (ch_rx_output_to_filter, ch_tx_output_to_filter) = mpsc::channel(100);

        let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

        // Задача создания запросов на основе входящих сообщений
        let task = tasks::InputRequest {
            address: self.address,
            buffer: buffer.clone(),
            ch_rx_msgbus_to_device,
            ch_tx_device_to_uart: ch_tx_device_to_uart.clone(),
            fn_msgs_to_buffer: self.fn_msgs_to_buffer,
            fn_buffer_to_request: self.fn_buffer_to_request,
        };
        join_set_spawn(&mut task_set, task.spawn());

        // Задача создания периодических запросов
        for periodic_request in self.periodic_requests {
            let task = tasks::PeriodicRequest {
                address: self.address,
                buffer: buffer.clone(),
                period: periodic_request.period,
                request: periodic_request.fn_request,
                ch_tx_device_to_uart: ch_tx_device_to_uart.clone(),
            };
            join_set_spawn(&mut task_set, task.spawn());
        }

        // Задача обработки ответа
        let task = tasks::Response {
            address: self.address,
            buffer: buffer.clone(),
            ch_rx_uart_to_device,
            ch_rx_output_to_filter,
            fn_response_to_buffer: self.fn_response_to_buffer,
            fn_buffer_to_msgs: self.fn_buffer_to_msgs,
        };
        join_set_spawn(&mut task_set, task.spawn());

        // Задачи фильтрации одинаковых сообщений
        let task = shared_tasks::filter_identical_data::FilterIdenticalData {
            input: ch_tx_output_to_filter,
            output: ch_tx_device_to_msgbus,
        };
        join_set_spawn(
            &mut task_set,
            task.spawn().map_err(super::Error::TaskFilterIdenticalData),
        );

        while let Some(res) = task_set.join_next().await {
            res??;
        }
        Ok(())
    }
}
