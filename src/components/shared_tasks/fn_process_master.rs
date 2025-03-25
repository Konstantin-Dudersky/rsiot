//! Запуск задач, общих для всех компонентов, выполняющих опрос устройств по шине

use futures::TryFutureExt;
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};

use crate::{
    components_config::master_device::{self, AddressBound, DeviceTrait, RequestResponseBound},
    executor::{join_set_spawn, CmpInOut},
    message::{Message, MsgDataBound},
};

use super::{filter_identical_data, mpsc_to_msgbus, msgbus_to_broadcast};

/// Запуск задач, общих для всех компонентов, выполняющих опрос устройств по шине
pub struct FnProcessMaster<'a, TMsg, TError, TFieldbusRequest, TFieldbusResponse, TAddress>
where
    TMsg: MsgDataBound + 'static,
    TError: Send + Sync + 'static,
    TAddress: AddressBound,
{
    /// Шина сообщений
    pub msg_bus: CmpInOut<TMsg>,

    /// Ёмкость очередей сообщений между задачами
    pub buffer_size: usize,

    /// Ссылка на коллекцию задач tokio
    pub task_set: &'a mut JoinSet<Result<(), TError>>,

    /// Ошибка msgbus_to_broadcast
    pub error_msgbus_to_broadcast: fn(msgbus_to_broadcast::Error) -> TError,

    /// Ошибка filter_identical_data
    pub error_filter: fn(filter_identical_data::Error) -> TError,

    /// Ошибка mpsc_to_msgbus
    pub error_mpsc_to_msgbus: fn(mpsc_to_msgbus::Error) -> TError,

    /// Ошибка master_device
    pub error_master_device: fn(master_device::Error) -> TError,

    /// Массив устройств
    pub devices: Vec<Box<dyn DeviceTrait<TMsg, TFieldbusRequest, TFieldbusResponse, TAddress>>>,
}

impl<TMsg, TError, TFieldbusRequest, TFieldbusResponse, TAddress>
    FnProcessMaster<'_, TMsg, TError, TFieldbusRequest, TFieldbusResponse, TAddress>
where
    TMsg: MsgDataBound + 'static,
    TError: Send + Sync + 'static,
    TFieldbusRequest: RequestResponseBound<TAddress> + 'static,
    TFieldbusResponse: RequestResponseBound<TAddress> + 'static,
    TAddress: 'static + AddressBound,
{
    /// Запуск задач.
    ///
    /// Возвращает кортеж с каналами передачи запросов / ответов
    pub fn spawn(
        self,
    ) -> (
        mpsc::Receiver<TFieldbusRequest>,
        broadcast::Sender<TFieldbusResponse>,
    ) {
        // Создание каналов передачи данных ------------------------------------------------------------

        // Канал передачи со входа компонента на устройства
        let (ch_tx_msgbus_to_devices, ch_rx_msgbus_to_devices) =
            broadcast::channel::<Message<TMsg>>(self.buffer_size);

        // Канал передачи данных из драйверов в канал SPI
        let (ch_tx_devices_to_fieldbus, ch_rx_devices_to_fieldbus) =
            mpsc::channel::<TFieldbusRequest>(self.buffer_size);

        // Канал передачи из канала SPI всем драйверам
        let (ch_tx_fieldbus_to_devices, ch_rx_fieldbus_to_devices) =
            broadcast::channel::<TFieldbusResponse>(self.buffer_size);

        // Канал передачи из устройств на фильтр
        let (ch_tx_devices_to_filter, ch_rx_devices_to_filter) =
            mpsc::channel::<Message<TMsg>>(self.buffer_size);

        // Канал передачи из фильтра на выход компонента
        let (ch_tx_filter_to_msgbus, ch_rx_filter_to_msgbus) =
            mpsc::channel::<Message<TMsg>>(self.buffer_size);

        // Передача входящих сообщений на драйвера устройств -------------------------------------------
        let task = msgbus_to_broadcast::MsgBusToBroadcast {
            msg_bus: self.msg_bus.clone(),
            output: ch_tx_msgbus_to_devices,
        };
        join_set_spawn(
            self.task_set,
            task.spawn().map_err(self.error_msgbus_to_broadcast),
        );

        // Задачи выполнения драйверов устройств -------------------------------------------------------
        for device in self.devices {
            let ch_rx_msgbus_to_devices = ch_rx_msgbus_to_devices.resubscribe();
            let ch_tx_device_to_fieldbus = ch_tx_devices_to_fieldbus.clone();
            let ch_rx_fieldbus_to_device = ch_rx_fieldbus_to_devices.resubscribe();
            let ch_tx_devices_to_filter = ch_tx_devices_to_filter.clone();
            join_set_spawn(
                self.task_set,
                device
                    .spawn(
                        ch_rx_msgbus_to_devices,
                        ch_tx_device_to_fieldbus,
                        ch_rx_fieldbus_to_device,
                        ch_tx_devices_to_filter,
                    )
                    .map_err(self.error_master_device),
            );
        }

        // Фильтрация одинаковых сообщений -------------------------------------------------------------
        let task = filter_identical_data::FilterIdenticalData {
            input: ch_rx_devices_to_filter,
            output: ch_tx_filter_to_msgbus,
        };
        join_set_spawn(self.task_set, task.spawn().map_err(self.error_filter));

        // Создаем исходящие сообщения -----------------------------------------------------------------
        let task = mpsc_to_msgbus::MpscToMsgBus {
            input: ch_rx_filter_to_msgbus,
            msg_bus: self.msg_bus.clone(),
        };
        join_set_spawn(
            self.task_set,
            task.spawn().map_err(self.error_mpsc_to_msgbus),
        );

        (ch_rx_devices_to_fieldbus, ch_tx_fieldbus_to_devices)
    }
}
