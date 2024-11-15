use std::sync::Arc;

use esp_idf_svc::hal::{
    gpio,
    peripheral::Peripheral,
    uart::{self, AsyncUartDriver, Uart},
};
use tokio::{sync::Mutex, task::JoinSet};

use crate::{
    components::cmp_esp_uart_slave::Parity,
    executor::{join_set_spawn, CmpInOut},
    message::MsgDataBound,
};

use super::{tasks, Config, RequestResponseBound};

pub async fn fn_process<TMsg, TUart, TPeripheral, TRequest, TResponse, TBufferData>(
    config: Config<TMsg, TUart, TPeripheral, TRequest, TResponse, TBufferData>,
    _msg_bus: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
    TUart: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Uart,
    TRequest: 'static + RequestResponseBound,
    TResponse: 'static + RequestResponseBound,
    TBufferData: 'static,
{
    println!("Starting UART loopback test");

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
        Option::<gpio::Gpio1>::None,
        &uart_config,
    )
    .unwrap();

    let buffer_data = config.buffer_data_default;
    let buffer_data = Arc::new(Mutex::new(buffer_data));

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    // Задача коммуникации по протоклу UART
    let task = tasks::UartComm {
        address: config.address,
        uart,
        fn_uart_comm: config.fn_uart_comm,
        buffer_data,
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Ждем выполнения задач
    while let Some(res) = task_set.join_next().await {
        res.unwrap().unwrap();
    }

    Ok(())
}
