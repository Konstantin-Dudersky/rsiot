use std::{sync::Arc, time::Duration};

use futures::TryFutureExt;
use tokio::{
    sync::{broadcast, mpsc, Mutex},
    task::JoinSet,
};

use crate::{
    components::shared_tasks,
    executor::{join_set_spawn, CmpInOut},
    message::MsgDataBound,
};

use super::{tasks, Config};

pub async fn fn_process<TMsg>(config: Config<TMsg>, msg_bus: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let serial_port_builder = serialport::new("", 0)
        .path(config.port)
        .baud_rate(config.baudrate.into())
        .data_bits(config.data_bits.into())
        .parity(config.parity.into())
        .stop_bits(config.stop_bits.into())
        .timeout(Duration::from_millis(100));

    let port = serial_port_builder.open().expect("Failed to open port");
    let port = Arc::new(Mutex::new(port));

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    // Канал передачи данных из драйверов в канал UART
    let (ch_tx_device_to_uart, ch_rx_device_to_uart) = mpsc::channel(1000);
    // Канал передачи из канала UART всем драйверам
    let (ch_tx_uart_to_device, ch_rx_uart_to_device) = broadcast::channel(1000);
    // Канал передачи из драйверов на фильтр исходящих сообщений
    let (ch_tx_device_to_filter, ch_rx_device_to_filter) = mpsc::channel(1000);
    // Канал передачи из фильтра сообщений на выход компонента
    let (ch_rx_filter_to_msgbus, ch_tx_filter_to_msgbus) = mpsc::channel(1000);

    // Задача записи в UART ------------------------------------------------------------------------
    let task = tasks::UartWrite {
        input: ch_rx_device_to_uart,
        port: port.clone(),
    };
    task_set.spawn_blocking(|| task.spawn());

    // Задача чтения из UART -----------------------------------------------------------------------
    let task = tasks::UartRead {
        output: ch_tx_uart_to_device,
        port: port.clone(),
    };
    task_set.spawn_blocking(|| task.spawn());

    // Задача выполнения драйверов устройств -------------------------------------------------------
    for device in config.devices {
        let ch_rx = ch_rx_uart_to_device.resubscribe();
        join_set_spawn(
            &mut task_set,
            device.spawn(
                ch_tx_device_to_uart.clone(),
                ch_rx,
                ch_tx_device_to_filter.clone(),
            ),
        );
    }

    // Задача фильтрации исходящих сообщений -------------------------------------------------------
    let task = shared_tasks::FilterIdenticalData {
        input: ch_rx_device_to_filter,
        output: ch_rx_filter_to_msgbus,
    };
    join_set_spawn(
        &mut task_set,
        task.spawn().map_err(super::Error::TaskFilterIdenticalData),
    );

    // Задача передачи сообщений на выход компонента -----------------------------------------------
    let task = shared_tasks::MpscToMsgBus {
        input: ch_tx_filter_to_msgbus,
        cmp_in_out: msg_bus,
    };
    join_set_spawn(
        &mut task_set,
        task.spawn().map_err(super::Error::TaskMpscToMsgBus),
    );

    // Ожидание выполнения -------------------------------------------------------------------------
    while let Some(res) = task_set.join_next().await {
        res.unwrap().unwrap();
    }

    Ok(())
}
