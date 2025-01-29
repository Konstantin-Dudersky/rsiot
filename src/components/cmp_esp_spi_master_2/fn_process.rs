use esp_idf_svc::hal::{
    peripheral::Peripheral,
    spi::{Spi, SpiAnyPins},
};
use futures::TryFutureExt;
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};

use crate::{
    components::shared_tasks,
    components_config::spi_master,
    executor::{join_set_spawn, CmpInOut},
    message::{Message, MsgDataBound, ServiceBound},
};

use super::{tasks, Config};

pub async fn fn_process<TMsg, TService, TSpi, TPeripheral>(
    config: Config<TMsg, TSpi, TPeripheral>,
    msg_bus: CmpInOut<TMsg, TService>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi + SpiAnyPins + 'static,
{
    const BUFFER_SIZE: usize = 500;

    let mut task_set = JoinSet::new();

    // Создание каналов передачи данных ------------------------------------------------------------

    // Канал передачи со входа компонента на устройства
    let (ch_tx_msgbus_to_devices, ch_rx_msgbus_to_devices) =
        broadcast::channel::<Message<TMsg>>(BUFFER_SIZE);

    // Канал передачи данных из драйверов в канал SPI
    let (ch_tx_devices_to_fieldbus, ch_rx_devices_to_fieldbus) =
        mpsc::channel::<spi_master::Request>(BUFFER_SIZE);

    // Канал передачи из канала SPI всем драйверам
    let (ch_tx_fieldbus_to_devices, ch_rx_fieldbus_to_devices) =
        broadcast::channel::<spi_master::Response>(BUFFER_SIZE);

    // Канал передачи из устройств на фильтр
    let (ch_tx_devices_to_filter, ch_rx_devices_to_filter) =
        mpsc::channel::<Message<TMsg>>(BUFFER_SIZE);

    // Канал передачи из фильтра на выход компонента
    let (ch_tx_filter_to_msgbus, ch_rx_filter_to_msgbus) =
        mpsc::channel::<Message<TMsg>>(BUFFER_SIZE);

    // Передача входящих сообщений на драйвера устройств -------------------------------------------
    let task = shared_tasks::msgbus_to_broadcast::MsgBusToBroadcast {
        msgbus: msg_bus.clone(),
        output: ch_tx_msgbus_to_devices,
    };
    join_set_spawn(
        &mut task_set,
        task.spawn().map_err(super::Error::TaskMsgbusToBroadcast),
    );

    // Задачи выполнения драйверов устройств -------------------------------------------------------
    for device in config.devices {
        let ch_rx_msgbus_to_devices = ch_rx_msgbus_to_devices.resubscribe();
        let ch_tx_device_to_fieldbus = ch_tx_devices_to_fieldbus.clone();
        let ch_rx_fieldbus_to_device = ch_rx_fieldbus_to_devices.resubscribe();
        let ch_tx_devices_to_filter = ch_tx_devices_to_filter.clone();
        join_set_spawn(
            &mut task_set,
            device
                .spawn(
                    ch_rx_msgbus_to_devices,
                    ch_tx_device_to_fieldbus,
                    ch_rx_fieldbus_to_device,
                    ch_tx_devices_to_filter,
                )
                .map_err(super::Error::DeviceError),
        );
    }

    // Коммуникация SPI ----------------------------------------------------------------------------
    let task = tasks::SpiComm {
        input: ch_rx_devices_to_fieldbus,
        output: ch_tx_fieldbus_to_devices,
        spi: config.spi,
        pin_miso: config.pin_miso,
        pin_mosi: config.pin_mosi,
        pin_sck: config.pin_sck,
        pin_cs0: config.pin_cs0,
        pin_cs1: config.pin_cs1,
        pin_cs2: config.pin_cs2,
        pin_cs3: config.pin_cs3,
        baudrate: config.baudrate,
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Фильтрация одинаковых сообщений -------------------------------------------------------------
    let task = shared_tasks::filter_identical_data::FilterIdenticalData {
        input: ch_rx_devices_to_filter,
        output: ch_tx_filter_to_msgbus,
    };
    join_set_spawn(
        &mut task_set,
        task.spawn().map_err(super::Error::TaskFilter),
    );

    // Создаем исходящие сообщения -----------------------------------------------------------------
    let task = shared_tasks::mpsc_to_msgbus::MpscToMsgBus {
        input: ch_rx_filter_to_msgbus,
        cmp_in_out: msg_bus,
    };
    join_set_spawn(
        &mut task_set,
        task.spawn().map_err(super::Error::TaskMpscToMsgBus),
    );

    while let Some(res) = task_set.join_next().await {
        res??
    }

    Ok(())
}
