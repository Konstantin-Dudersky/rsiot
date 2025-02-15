use std::sync::Arc;

use esp_idf_svc::hal::gpio::{AnyIOPin, PinDriver};
use esp_idf_svc::hal::{
    gpio,
    peripheral::Peripheral,
    uart::{self, AsyncUartDriver, Uart},
};
use futures::TryFutureExt;
use tokio::{
    sync::{mpsc, Mutex},
    task::JoinSet,
};
use tracing::info;

use crate::components_config::uart_general::Parity;
use crate::{
    components::shared_tasks::{filter_identical_data, mpsc_to_msgbus},
    executor::{join_set_spawn, CmpInOut},
    message::{MsgDataBound, ServiceBound},
};

use super::{tasks, Config};

pub async fn fn_process<TMsg, TService, TUart, TPeripheral, TBufferData, const MESSAGE_LEN: usize>(
    config: Config<TMsg, TUart, TPeripheral, TBufferData, MESSAGE_LEN>,
    msg_bus: CmpInOut<TMsg, TService>,
) -> super::Result<()>
where
    TMsg: 'static + MsgDataBound,
    TService: ServiceBound + 'static,
    TUart: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Uart,
    TBufferData: 'static,
{
    info!("Starting UART loopback test");

    let uart_config = uart::config::Config::new()
        .baudrate(config.baudrate.into())
        .data_bits(config.data_bits.into())
        .stop_bits(config.stop_bits.into());
    let uart_config = match config.parity {
        Parity::None => uart_config.parity_none(),
        Parity::Even => uart_config.parity_even(),
        Parity::Odd => uart_config.parity_odd(),
    };

    let uart = AsyncUartDriver::new(
        config.uart,
        config.pin_tx,
        config.pin_rx,
        Option::<gpio::Gpio0>::None,
        // Option::<gpio::Gpio1>::None,
        Option::<AnyIOPin>::None,
        // Some(config.pin_rts),
        &uart_config,
    )
    .unwrap();

    let pin_rts = PinDriver::output(config.pin_rts).unwrap();

    let buffer_data = config.buffer_data_default;
    let buffer_data = Arc::new(Mutex::new(buffer_data));

    let (ch_tx_output_to_filter, ch_rx_output_to_filter) = mpsc::channel(100);
    let (ch_tx_filter_to_msgbus, ch_rx_filter_to_msgbus) = mpsc::channel(100);

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    // Задача обработки входящих сообщений
    let task = tasks::Input {
        msg_bus: msg_bus.clone(),
        fn_input: config.fn_input,
        buffer_data: buffer_data.clone(),
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Задача коммуникации по протоклу UART
    let task = tasks::UartComm {
        address: config.address,
        uart,
        pin_rts,
        fn_uart_comm: config.fn_uart_comm,
        buffer_data: buffer_data.clone(),
        delay_between_read_and_write: config.delay_between_read_and_write,
    };
    join_set_spawn(&mut task_set, task.spawn::<MESSAGE_LEN>());

    // Задача генерирования исходящих сообщений
    let task = tasks::Output {
        output: ch_tx_output_to_filter,
        buffer_data: buffer_data.clone(),
        fn_output: config.fn_output,
        fn_output_period: config.fn_output_period,
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Задача фильтрации исходящих сообщений
    let task = filter_identical_data::FilterIdenticalData {
        input: ch_rx_output_to_filter,
        output: ch_tx_filter_to_msgbus,
    };
    join_set_spawn(
        &mut task_set,
        task.spawn().map_err(super::Error::TaskFilterIdenticalData),
    );

    // Задача передачи сообщений в шину
    let task = mpsc_to_msgbus::MpscToMsgBus {
        input: ch_rx_filter_to_msgbus,
        cmp_in_out: msg_bus.clone(),
    };
    join_set_spawn(
        &mut task_set,
        task.spawn().map_err(super::Error::TaskMpscToMsgbus),
    );

    // Ждем выполнения задач
    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Ok(())
}
