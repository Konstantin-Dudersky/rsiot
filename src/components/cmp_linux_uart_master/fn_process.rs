use tokio::task::JoinSet;

use crate::{
    components::shared_tasks::fn_process_master::FnProcessMaster,
    executor::CmpInOut,
    message::{MsgDataBound, ServiceBound},
};

use super::{uart_comm::UartComm, Config};

pub async fn fn_process<TMsg, TService>(
    config: Config<TMsg>,
    msg_bus: CmpInOut<TMsg, TService>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
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
        pin_rts: config.pin_rts,
        ch_rx_devices_to_fieldbus,
        ch_tx_fieldbus_to_devices,
        port: config.port,
        timeout: config.timeout,
        baudrate: config.baudrate,
        data_bits: config.data_bits,
        parity: config.parity,
        stop_bits: config.stop_bits,
        gpio_chip: config.gpio_chip,
    };
    task_set.spawn_blocking(move || task.spawn());

    // Ожидание выполнения -------------------------------------------------------------------------
    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Ok(())
}
