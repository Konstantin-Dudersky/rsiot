use esp_idf_svc::hal::{
    gpio,
    peripheral::Peripheral,
    uart::{self, AsyncUartDriver, Uart},
};
use tokio::task::JoinSet;

use crate::{
    components::shared_tasks::fn_process_master::FnProcessMaster,
    components_config::uart_general::Parity,
    executor::{MsgBusLinker, join_set_spawnMsgBusLinker},
    message::{MsgDataBound, ServiceBound},
};

use super::{Config, uart_comm::UartComm};

pub async fn fn_process<TMsg, TService, TUart, TPeripheral>(
    config: Config<TMsg, TUart, TPeripheral>,
    msg_bus: MsgBusLinker<TMsg, TService>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    TUart: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Uart,
{
    let uart_config = uart::config::Config::new()
        .baudrate(config.baudrate.into())
        .data_bits(config.data_bits.into())
        .stop_bits(config.stop_bits.into())
        .mode(uart::config::Mode::RS485HalfDuplex);
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
        Some(config.pin_rts),
        &uart_config,
    )
    .unwrap();

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    const BUFFER_SIZE: usize = 1000;

    let config_fn_process_master = FnProcessMaster {
        msg_bus,
        buffer_size: BUFFER_SIZE,
        task_set: &mut task_set,
        error_msgbus_to_broadcast: super::Error::TaskMsgbusToBroadcast,
        error_filter: super::Error::TaskFilterIdenticalData,
        error_mpsc_to_msgbus: super::Error::TaskMpscToMsgBus,
        error_master_device: super::Error::Device,
        devices: config.devices,
    };

    let (ch_rx_devices_to_fieldbus, ch_tx_fieldbus_to_devices) = config_fn_process_master.spawn();

    // Коммуникация UART
    //
    // Запускаем в отдельном потоке, чтобы не было увеличенного времени ожидания в точках await
    let task = UartComm {
        ch_rx_devices_to_fieldbus,
        ch_tx_fieldbus_to_devices,
        uart_driver: uart,
        timeout: config.timeout,
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Ожидание выполнения -------------------------------------------------------------------------
    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Ok(())
}
