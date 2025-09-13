use tokio::task::JoinSet;

use crate::{
    components::shared_tasks::cmp_can_general::CanGeneralTasks,
    components_config::can_general::BufferBound, executor::CmpInOut, message::MsgDataBound,
};

use super::{Config, Error};

pub async fn fn_process<TMsg, TBuffer>(
    config: Config<TMsg, TBuffer>,
    msg_bus: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: 'static + MsgDataBound,
    TBuffer: 'static + BufferBound,
{
    let mut task_set: JoinSet<Result<(), Error>> = JoinSet::new();

    let (ch_rx_send_to_can, ch_tx_recv_from_can) = CanGeneralTasks {
        msg_bus,
        buffer_default: config.buffer_default,
        buffer_size: 1000,
        task_set: &mut task_set,
        fn_input: config.fn_input,
        period: config.period,
        fn_periodic: config.fn_periodic,
        fn_output: config.fn_output,
        error_task_end_input: || Error::TaskEndInput,
        error_task_end_output: || Error::TaskEndOutput,
        error_tokio_mpsc_send: || Error::TokioSyncMpscSend,
    }
    .spawn();

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Err(Error::TaskEnd)
}
