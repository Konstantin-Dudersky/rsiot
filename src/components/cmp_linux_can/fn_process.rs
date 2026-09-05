use futures::TryFutureExt;
use tokio::{sync::broadcast, task::JoinSet};

use crate::{
    components::{
        cmp_linux_can::CanFrame,
        shared_tasks::{self, cmp_can_general::CanGeneralTasks},
    },
    executor::{MsgBusLinker, join_set_spawn},
    message::MsgDataBound,
};

use super::{
    Config, Error, task_interface_info::InterfaceInfo, task_setup_send_recv::TaskSetupSendRecv,
};

pub async fn fn_process<TMsg, TFnInput>(
    config: Config<TMsg, TFnInput>,
    msgbus_linker: MsgBusLinker<TMsg>,
) -> super::Result<()>
where
    TMsg: 'static + MsgDataBound,
    TFnInput: 'static + Fn(&TMsg) -> anyhow::Result<Option<Vec<CanFrame>>> + Send,
{
    let mut task_set: JoinSet<Result<(), Error>> = JoinSet::new();

    // Общие задачи обмена по шине CAN
    let (ch_rx_send_to_can, ch_tx_recv_from_can) = CanGeneralTasks {
        msgbus_linker,
        task_set: &mut task_set,
        fn_input: config.fn_input,
        fn_output: config.fn_output,
        error_task_end_input: || Error::TaskEndInput,
        error_task_end_output: || Error::TaskEndOutput,
        error_tokio_mpsc_send: || Error::TokioSyncMpscSend,
    }
    .spawn();

    let (ch_tx, ch_rx) = broadcast::channel::<CanFrame>(100000);

    let task = shared_tasks::mpsc_to_broadcast::Task {
        input: ch_rx_send_to_can,
        output: ch_tx,
    };
    join_set_spawn(
        &mut task_set,
        "cmp_linux_can | mpsc_to_broadcast",
        task.spawn().map_err(Error::TaskMpscToBroadcast),
    );

    let task = TaskSetupSendRecv {
        input: ch_rx,
        output: ch_tx_recv_from_can,
        ifname: config.ifname.clone(),
        can_settings: config.can_settings,
        filters: config.filters,
        async_version: config.async_version,
    };
    join_set_spawn(
        &mut task_set,
        "cmp_linux_can | setup_send_recv",
        task.spawn(),
    );

    // let task = InterfaceInfo {
    //     ifname: config.ifname,
    //     period: Duration::from_millis(1000),
    // };
    // join_set_spawn(
    //     &mut task_set,
    //     "cmp_linux_can | interface_info",
    //     task.spawn(),
    // );

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Err(Error::TaskEnd)
}
