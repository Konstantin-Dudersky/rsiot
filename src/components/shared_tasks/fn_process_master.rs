//! Запуск задач, общих для всех компонентов, выполняющих опрос устройств по шине

use futures::TryFutureExt;
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};

use crate::{
    components_config::master_device::{self, DeviceTrait, RequestResponseBound},
    executor::{join_set_spawn, CmpInOut},
    message::{Message, MsgDataBound, ServiceBound},
};

use super::{filter_identical_data, mpsc_to_msgbus, msgbus_to_broadcast};

/// Запуск задач, общих для всех компонентов, выполняющих опрос устройств по шине
pub struct FnProcessMaster<'a, TMsg, TService, TError, TFieldbusRequest, TFieldbusResponse>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    TError: Send + Sync + 'static,
{
    pub msg_bus: CmpInOut<TMsg, TService>,
    pub buffer_size: usize,
    pub task_set: &'a mut JoinSet<Result<(), TError>>,
    pub error_msgbus_to_broadcast: fn(msgbus_to_broadcast::Error) -> TError,
    pub error_filter: fn(filter_identical_data::Error) -> TError,
    pub error_mpsc_to_msgbus: fn(mpsc_to_msgbus::Error) -> TError,
    pub error_master_device: fn(master_device::Error) -> TError,
    pub devices: Vec<Box<dyn DeviceTrait<TMsg, TFieldbusRequest, TFieldbusResponse>>>,
}

impl<TMsg, TService, TError, TFieldbusRequest, TFieldbusResponse>
    FnProcessMaster<'_, TMsg, TService, TError, TFieldbusRequest, TFieldbusResponse>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
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
            msgbus: self.msg_bus.clone(),
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
            cmp_in_out: self.msg_bus.clone(),
        };
        join_set_spawn(
            self.task_set,
            task.spawn().map_err(self.error_mpsc_to_msgbus),
        );

        (ch_rx_devices_to_fieldbus, ch_tx_fieldbus_to_devices)
    }
}
