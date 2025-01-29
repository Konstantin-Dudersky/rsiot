#![allow(clippy::module_inception)]

use std::sync::Arc;

use futures::TryFutureExt;
use tokio::{
    sync::{broadcast, mpsc, Mutex},
    task::JoinSet,
};

use crate::{components::shared_tasks, message::Message};
use crate::{executor::join_set_spawn, message::MsgDataBound};

use super::{config::*, tasks, BufferBound, RequestResponseBound};

/// Базовое устройство для опроса по шине
pub struct DeviceBase<TMsg, TFieldbusRequest, TFieldbusResponse, TBuffer>
where
    TFieldbusRequest: RequestResponseBound,
{
    /// Адрес
    pub address: u8,

    /// Запросы при инициализации устройства
    pub fn_init_requests: fn() -> Vec<TFieldbusRequest>,

    /// Периодические запросы
    pub periodic_requests: Vec<ConfigPeriodicRequest<TFieldbusRequest, TBuffer>>,

    /// Обновление буфера на основе входящих сообщений
    ///
    /// Обычно соответствует параметру fn_input конфигурации устройства
    ///
    /// # Пример
    ///
    /// ```rust
    /// let msgs = [Custom::AllInputs(buffer.all_inputs)];
    /// let msgs = msgs.iter().map(|m| Message::new_custom(*m)).collect();
    /// msgs
    /// ```
    pub fn_msgs_to_buffer: fn(&Message<TMsg>, &mut TBuffer),

    pub fn_buffer_to_request: fn(&TBuffer) -> Vec<TFieldbusRequest>,
    pub fn_response_to_buffer: fn(TFieldbusResponse, &mut TBuffer),

    /// Функция создания сообщений на основе буфера
    ///
    /// Обычно соответствует параметру fn_output кофигурации устройства
    ///
    /// Пример:
    ///
    /// ```rust
    /// |buffer| {vec![]}
    /// ```
    pub fn_buffer_to_msgs: fn(&TBuffer) -> Vec<Message<TMsg>>,

    /// Значения в буфере при инициализации
    pub buffer_default: TBuffer,
}

impl<TMsg, TRequest, TResponse, TBuffer> DeviceBase<TMsg, TRequest, TResponse, TBuffer>
where
    TRequest: 'static + RequestResponseBound,
    TResponse: 'static + RequestResponseBound,
    TMsg: MsgDataBound + 'static,
    TBuffer: 'static + BufferBound,
{
    /// Запустить работу
    pub async fn spawn(
        self,
        ch_rx_msgbus_to_device: broadcast::Receiver<Message<TMsg>>,
        ch_tx_device_to_fieldbus: mpsc::Sender<TRequest>,
        ch_rx_fieldbus_to_device: broadcast::Receiver<TResponse>,
        ch_tx_device_to_msgbus: mpsc::Sender<Message<TMsg>>,
    ) -> super::Result<()> {
        let buffer = self.buffer_default;
        let buffer = Arc::new(Mutex::new(buffer));

        let (ch_tx_output_to_filter, ch_rx_output_to_filter) = mpsc::channel(100);

        let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

        // Задача выполнения первоначальных запросов
        //
        // Приостанавливаем выполнение, пока не будет выполнена задача
        let task = tasks::InitRequest {
            address: self.address,
            fn_init_requests: self.fn_init_requests,
            ch_tx_device_to_fieldbus: ch_tx_device_to_fieldbus.clone(),
        };
        task.spawn().await.unwrap();

        // Задача создания запросов на основе входящих сообщений
        let task = tasks::InputRequest {
            address: self.address,
            buffer: buffer.clone(),
            ch_rx_msgbus_to_device,
            ch_tx_device_to_fieldbus: ch_tx_device_to_fieldbus.clone(),
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
                fn_request: periodic_request.fn_requests,
                ch_tx_device_to_fieldbus: ch_tx_device_to_fieldbus.clone(),
            };
            join_set_spawn(&mut task_set, task.spawn());
        }

        // Задача обработки ответа
        let task = tasks::Response {
            address: self.address,
            buffer: buffer.clone(),
            ch_rx_fieldbus_to_device,
            ch_tx_output_to_filter,
            fn_response_to_buffer: self.fn_response_to_buffer,
            fn_buffer_to_msgs: self.fn_buffer_to_msgs,
        };
        join_set_spawn(&mut task_set, task.spawn());

        // Задачи фильтрации одинаковых сообщений
        let task = shared_tasks::filter_identical_data::FilterIdenticalData {
            input: ch_rx_output_to_filter,
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

impl<TMsg, TFieldbusRequest, TFieldbusResponse, TBuffer> Default
    for DeviceBase<TMsg, TFieldbusRequest, TFieldbusResponse, TBuffer>
where
    TFieldbusRequest: RequestResponseBound,
    TBuffer: Default,
{
    fn default() -> Self {
        DeviceBase {
            address: 0,
            fn_init_requests: Vec::new,
            periodic_requests: vec![],
            fn_msgs_to_buffer: |_, _| (),
            fn_buffer_to_request: |_| vec![],
            fn_response_to_buffer: |_, _| (),
            fn_buffer_to_msgs: |_| vec![],
            buffer_default: Default::default(),
        }
    }
}
