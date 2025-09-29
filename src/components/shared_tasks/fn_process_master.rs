//! Запуск задач, общих для всех компонентов, выполняющих опрос устройств по шине

use std::ops::Index;

use futures::TryFutureExt;
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};

use crate::{
    components::{cmp_derive::Error, shared_tasks::mpsc_to_msgbus_new},
    components_config::master_device::{
        self, DeviceTrait, FieldbusRequestWithIndex, FieldbusResponseWithIndex,
        RequestResponseBound,
    },
    executor::{CmpInOut, MsgBusInput, MsgBusOutput, join_set_spawn},
    message::{Message, MsgDataBound},
};

use super::{filter_identical_data, msgbus_to_broadcast};

/// Запуск задач, общих для всех компонентов, выполняющих опрос устройств по шине
pub struct FnProcessMaster<'a, TMsg, TError, TFieldbusRequest, TFieldbusResponse>
where
    TMsg: MsgDataBound + 'static,
    TError: Send + Sync + 'static,
{
    /// Шина сообщений - входящие сообщения
    pub input: MsgBusInput<TMsg>,

    /// Шина сообщений - исходящие сообщения
    pub output: MsgBusOutput<TMsg>,

    /// Ссылка на коллекцию задач tokio
    pub task_set: &'a mut JoinSet<Result<(), TError>>,

    /// Ошибка filter_identical_data
    pub error_filter: fn(filter_identical_data::Error) -> TError,

    /// Ошибка mpsc_to_msgbus
    pub error_mpsc_to_msgbus: fn(mpsc_to_msgbus_new::Error) -> TError,

    /// Ошибка master_device
    pub error_master_device: fn(master_device::Error) -> TError,

    /// Ошибка tokio::mpsc::send
    pub error_tokiompscsend: fn() -> TError,

    /// Массив устройств
    pub devices: Vec<Box<dyn DeviceTrait<TMsg, TFieldbusRequest, TFieldbusResponse>>>,
}

impl<TMsg, TError, TFieldbusRequest, TFieldbusResponse>
    FnProcessMaster<'_, TMsg, TError, TFieldbusRequest, TFieldbusResponse>
where
    TMsg: MsgDataBound + 'static,
    TError: Send + Sync + 'static,
    TFieldbusRequest: RequestResponseBound + 'static,
    TFieldbusResponse: RequestResponseBound + 'static,
{
    /// Запуск задач.
    ///
    /// Возвращает кортеж с каналами передачи запросов / ответов
    pub fn spawn(
        self,
    ) -> (
        mpsc::Receiver<FieldbusRequestWithIndex<TFieldbusRequest>>,
        mpsc::Sender<FieldbusResponseWithIndex<TFieldbusResponse>>,
    ) {
        let devices_count = self.devices.len();
        let buffer_size = self.output.max_capacity();

        // Создание каналов передачи данных --------------------------------------------------------

        // Каналы передачи запросов из устройств в задачи добавления индекса
        let mut ch_tx_device_to_addindex = vec![];
        let mut ch_rx_device_to_addindex = vec![];

        for _ in 0..devices_count {
            let (ch_tx, ch_rx) = mpsc::channel::<TFieldbusRequest>(buffer_size);
            ch_tx_device_to_addindex.push(ch_tx);
            ch_rx_device_to_addindex.push(ch_rx);
        }

        // Канал передачи запросов из задач добавления индекса в шину
        let (ch_tx_addindex_to_fieldbus, ch_rx_addindex_to_fieldbus) =
            mpsc::channel::<FieldbusRequestWithIndex<TFieldbusRequest>>(buffer_size);

        // Канал передачи ответов из шины в задачу разделения ответов для устройств
        let (ch_tx_fieldbus_to_split, ch_rx_fieldbus_to_split) =
            mpsc::channel::<FieldbusResponseWithIndex<TFieldbusResponse>>(buffer_size);

        // Каналы передачи ответов из задачи разделения на устройства
        let mut ch_tx_split_to_devices = vec![];
        let mut ch_rx_split_to_devices = vec![];
        for _ in 0..devices_count {
            let (ch_tx, ch_rx) = mpsc::channel::<TFieldbusResponse>(buffer_size);
            ch_tx_split_to_devices.push(ch_tx);
            ch_rx_split_to_devices.push(Some(ch_rx));
        }

        // Канал передачи сообщений из устройств на фильтр
        let (ch_tx_devices_to_filter, ch_rx_devices_to_filter) =
            mpsc::channel::<Message<TMsg>>(buffer_size);

        // Канал передачи сообщений из фильтра на выход компонента
        let (ch_tx_filter_to_msgbus, ch_rx_filter_to_msgbus) =
            mpsc::channel::<Message<TMsg>>(buffer_size);

        // Задачи выполнения устройств -------------------------------------------------------------
        let mut input_vec = vec![];
        for _ in 0..self.devices.len() - 1 {
            input_vec.push(self.input.clone());
        }
        input_vec.push(self.input);

        for (index, device) in self.devices.into_iter().enumerate() {
            let ch_rx_msgbus_to_devices = input_vec[index].clone();
            let ch_tx_device_to_addindex = ch_tx_device_to_addindex[index].clone();
            let Some(ch_rx_fieldbus_to_device) = ch_rx_split_to_devices[index].take() else {
                panic!("Error configuration in fn_process_master");
            };
            let ch_tx_devices_to_filter = ch_tx_devices_to_filter.clone();
            let task = device.spawn(
                ch_rx_msgbus_to_devices,
                ch_tx_device_to_addindex,
                ch_rx_fieldbus_to_device,
                ch_tx_devices_to_filter,
            );
            join_set_spawn(
                self.task_set,
                "fn_process_master | device",
                task.map_err(self.error_master_device),
            );
        }

        // Задачи добавления индекса
        for (device_index, ch_rx) in ch_rx_device_to_addindex.into_iter().enumerate() {
            let task = AddIndex {
                input: ch_rx,
                output: ch_tx_addindex_to_fieldbus.clone(),
                device_index,
                error_tokiompscsend: self.error_tokiompscsend,
            };
            join_set_spawn(self.task_set, "fn_process_master | add_index", task.spawn());
        }

        // Задача разделения ответов
        let task = SplitResponses {
            input: ch_rx_fieldbus_to_split,
            output: ch_tx_split_to_devices,
            error_tokiompscsend: self.error_tokiompscsend,
        };
        join_set_spawn(
            self.task_set,
            "fn_process_master | split_responses",
            task.spawn(),
        );

        // Фильтрация одинаковых сообщений ---------------------------------------------------------
        let task = filter_identical_data::FilterIdenticalData {
            input: ch_rx_devices_to_filter,
            output: ch_tx_filter_to_msgbus,
        };
        join_set_spawn(
            self.task_set,
            "fn_process_master | filter_identical_data",
            task.spawn().map_err(self.error_filter),
        );

        // Создаем исходящие сообщения -------------------------------------------------------------
        let task = mpsc_to_msgbus_new::MpscToMsgBus {
            input: ch_rx_filter_to_msgbus,
            output: self.output,
        };
        join_set_spawn(
            self.task_set,
            "fn_process_master | mpsc_to_msgbus",
            task.spawn().map_err(self.error_mpsc_to_msgbus),
        );

        (ch_rx_addindex_to_fieldbus, ch_tx_fieldbus_to_split)
    }
}

struct AddIndex<TFieldbusRequest, TError>
where
    TFieldbusRequest: RequestResponseBound,
{
    pub input: mpsc::Receiver<TFieldbusRequest>,
    pub output: mpsc::Sender<FieldbusRequestWithIndex<TFieldbusRequest>>,
    pub device_index: usize,
    pub error_tokiompscsend: fn() -> TError,
}
impl<TFieldbusRequest, TError> AddIndex<TFieldbusRequest, TError>
where
    TFieldbusRequest: RequestResponseBound,
{
    pub async fn spawn(mut self) -> Result<(), TError> {
        while let Some(request) = self.input.recv().await {
            let request_with_index = FieldbusRequestWithIndex {
                device_index: self.device_index,
                request,
            };
            self.output
                .send(request_with_index)
                .await
                .map_err(|_| (self.error_tokiompscsend)())?;
        }
        Ok(())
    }
}

struct SplitResponses<TFieldbusResponse, TError>
where
    TFieldbusResponse: RequestResponseBound,
{
    pub input: mpsc::Receiver<FieldbusResponseWithIndex<TFieldbusResponse>>,
    pub output: Vec<mpsc::Sender<TFieldbusResponse>>,
    pub error_tokiompscsend: fn() -> TError,
}
impl<TFieldbusResponse, TError> SplitResponses<TFieldbusResponse, TError>
where
    TFieldbusResponse: RequestResponseBound,
{
    pub async fn spawn(mut self) -> Result<(), TError> {
        while let Some(response_with_index) = self.input.recv().await {
            let device_index = response_with_index.device_index;
            let response = response_with_index.response;
            self.output[device_index]
                .send(response)
                .await
                .map_err(|_| (self.error_tokiompscsend)())?;
        }
        Ok(())
    }
}
