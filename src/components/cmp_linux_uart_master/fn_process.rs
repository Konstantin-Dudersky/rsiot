use tokio::task::JoinSet;

use crate::{
    components::shared_tasks::fn_process_master::FnProcessMaster, executor::CmpInOut,
    message::MsgDataBound,
};

use super::{uart_comm::UartComm, Config};

pub async fn fn_process<TMsg>(config: Config<TMsg>, msg_bus: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
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
        error_tokiompscsend: || super::Error::TokioSyncMpscSend,
        devices: config.devices,
    };

    let (ch_rx_addindex_to_fieldbus, ch_tx_fieldbus_to_split) = config_fn_process_master.spawn();

    // Коммуникация UART
    //
    // Запускаем в отдельном потоке, чтобы не было увеличенного времени ожидания в точках await
    let task = UartComm {
        pin_rts: config.pin_rts,
        ch_rx_addindex_to_fieldbus,
        ch_tx_fieldbus_to_split,
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
