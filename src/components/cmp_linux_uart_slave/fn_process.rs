use tokio::task::JoinSet;

use crate::{executor::MsgBusLinker, message::MsgDataBound};

use super::{Config, Error, task_serial_comm::TaskSerialComm};

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    msgbus_linker: MsgBusLinker<TMsg>,
) -> super::Result<()>
where
    TMsg: 'static + MsgDataBound,
{
    let mut task_set: JoinSet<Result<(), Error>> = JoinSet::new();

    let task = TaskSerialComm {
        output: msgbus_linker.output(),
        port: config.port,
        baudrate: config.baudrate,
        data_bits: config.data_bits,
        parity: config.parity,
        stop_bits: config.stop_bits,
        timeout: config.timeout,
        fn_output: config.fn_output,
    };

    task_set.spawn_blocking(move || task.spawn());

    msgbus_linker.close();

    // Ожидание выполнения -------------------------------------------------------------------------
    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Err(Error::FnProcessEnd)
}
