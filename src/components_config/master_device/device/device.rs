#![allow(clippy::module_inception)]

use std::{sync::Arc, time::Duration};

use futures::TryFutureExt;
use tokio::{
    sync::{Mutex, mpsc},
    task::JoinSet,
};

use crate::{components::shared_tasks, executor::MsgBusInput, message::Message};
use crate::{executor::join_set_spawn, message::MsgDataBound};

use super::{BufferBound, RequestResponseBound, config::*, tasks};

/// Базовое устройство для опроса по шине
pub struct DeviceBase<TMsg, TFieldbusRequest, TFieldbusResponse, TBuffer>
where
    TFieldbusRequest: RequestResponseBound,
{
    /// Запросы при инициализации устройства
    pub fn_init_requests: fn(&TBuffer) -> Vec<TFieldbusRequest>,

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
    pub fn_msgs_to_buffer: fn(&TMsg, &mut TBuffer),

    /// Периодическое формирование запросов на основе fn_buffer_to_request
    pub buffer_to_request_period: Duration,

    /// Преобразование данных из буфера в массив запросов на шине
    ///
    /// Вызывается несколькими способами:
    ///
    /// - при обновлении буфера на основе входящих сообщений функцией fn_msgs_to_buffer
    ///
    /// - при расшифровке ответа от устройства, при возвращении true из функции fn_response_to_buffer
    ///
    /// - периодически с периодом buffer_to_request_period
    pub fn_buffer_to_request: fn(&TBuffer) -> anyhow::Result<Vec<TFieldbusRequest>>,

    /// Обновление буфера на основе данных, полученных с устройства
    pub fn_response_to_buffer: fn(TFieldbusResponse, &mut TBuffer) -> anyhow::Result<bool>,

    /// Функция создания сообщений на основе буфера
    ///
    /// Обычно соответствует параметру fn_output кофигурации устройства
    ///
    /// Пример:
    ///
    /// ```rust
    /// |buffer| {vec![]}
    /// ```
    pub fn_buffer_to_msgs: fn(&mut TBuffer) -> Vec<TMsg>,

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
        id: impl AsRef<str>,
        ch_rx_msgbus_to_device: MsgBusInput<TMsg>,
        ch_tx_device_to_fieldbus: mpsc::Sender<TRequest>,
        ch_rx_fieldbus_to_device: mpsc::Receiver<TResponse>,
        ch_tx_device_to_msgbus: mpsc::Sender<Message<TMsg>>,
    ) -> super::Result<()> {
        let buffer = self.buffer_default;
        let buffer = Arc::new(Mutex::new(buffer));

        let (ch_tx_buffer, ch_rx_buffer) = mpsc::channel::<()>(100);
        let (ch_tx_request, ch_rx_request) = mpsc::channel::<TRequest>(100);
        let (ch_tx_output_to_filter, ch_rx_output_to_filter) = mpsc::channel::<Message<TMsg>>(500);

        let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

        // Задача выполнения первоначальных запросов
        //
        // Приостанавливаем выполнение, пока не будет выполнена задача
        let task = tasks::InitRequest {
            buffer: buffer.clone(),
            fn_init_requests: self.fn_init_requests,
            ch_tx_request: ch_tx_request.clone(),
        };
        task.spawn().await?;

        // Задача создания периодических запросов
        for periodic_request in self.periodic_requests {
            let task = tasks::PeriodicRequest {
                buffer: buffer.clone(),
                period: periodic_request.period,
                fn_request: periodic_request.fn_requests,
                ch_tx_request: ch_tx_request.clone(),
            };
            join_set_spawn(
                &mut task_set,
                format!("master_device | periodic_request | {}", id.as_ref()),
                task.spawn(),
            );
        }

        // Задача создания запросов на основе входящих сообщений
        let task = tasks::InputRequest {
            buffer: buffer.clone(),
            ch_rx_msgbus_to_device,
            ch_tx_buffer: ch_tx_buffer.clone(),
            fn_msgs_to_buffer: self.fn_msgs_to_buffer,
        };
        join_set_spawn(
            &mut task_set,
            format!("master_device | input_request | {}", id.as_ref()),
            task.spawn(),
        );

        // Задача периодического формирования запросов на основе буфера
        let task = tasks::BufferPeriodic {
            ch_tx_buffer: ch_tx_buffer.clone(),
            period: self.buffer_to_request_period,
        };
        join_set_spawn(
            &mut task_set,
            format!("master_device | buffer_periodic | {}", id.as_ref()),
            task.spawn(),
        );

        // Задача формирования запросов на основе буфера
        let task = tasks::BufferToRequests {
            buffer: buffer.clone(),
            ch_rx_buffer,
            ch_tx_request: ch_tx_request.clone(),
            fn_buffer_to_request: self.fn_buffer_to_request,
        };
        join_set_spawn(
            &mut task_set,
            format!("master_device | buffer_to_requests | {}", id.as_ref()),
            task.spawn(),
        );

        // Задача отправки запросов
        let task = tasks::Request {
            ch_rx_request,
            ch_tx_device_to_fieldbus,
        };
        join_set_spawn(
            &mut task_set,
            format!("master_device | request | {}", id.as_ref()),
            task.spawn(),
        );

        // Задача обработки ответа
        let task = tasks::Response {
            buffer: buffer.clone(),
            ch_rx_fieldbus_to_device,
            ch_tx_output_to_filter,
            ch_tx_buffer: ch_tx_buffer.clone(),
            fn_response_to_buffer: self.fn_response_to_buffer,
            fn_buffer_to_msgs: self.fn_buffer_to_msgs,
        };
        join_set_spawn(
            &mut task_set,
            format!("master_device | response | {}", id.as_ref()),
            task.spawn(),
        );

        // Задачи фильтрации одинаковых сообщений
        let task = shared_tasks::filter_identical_data::FilterIdenticalData {
            input: ch_rx_output_to_filter,
            output: ch_tx_device_to_msgbus,
        };
        join_set_spawn(
            &mut task_set,
            format!("master_device | filter_identical_data | {}", id.as_ref()),
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
            fn_init_requests: |_| vec![],
            periodic_requests: vec![],
            fn_msgs_to_buffer: |_, _| (),
            buffer_to_request_period: Duration::from_millis(1000),
            fn_buffer_to_request: |_| Ok(vec![]),
            fn_response_to_buffer: |_, _| Ok(false),
            fn_buffer_to_msgs: |_| vec![],
            buffer_default: Default::default(),
        }
    }
}
