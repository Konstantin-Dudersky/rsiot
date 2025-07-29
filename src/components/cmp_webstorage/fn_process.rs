use futures::TryFutureExt;
use tokio::{sync::mpsc::channel, task::JoinSet};

use crate::{
    components::shared_tasks,
    executor::{join_set_spawn, CmpInOut},
    message::*,
};

use super::{tasks, Config};

pub async fn fn_process<TMsg>(config: Config<TMsg>, msg_bus: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    let channel_capacity = msg_bus.max_capacity();

    let (ch_tx_msgbus_to_input, ch_rx_msgbus_to_input) = channel(channel_capacity);
    let (ch_tx_input_to_output, ch_rx_input_to_output) = channel(channel_capacity);
    let (ch_tx_output_to_msgbus, ch_rx_output_to_msgbus) = channel(channel_capacity);

    // Со входа компонента на задачу Input
    let task_0 = shared_tasks::msgbus_to_mpsc::MsgBusToMpsc {
        msg_bus: msg_bus.clone(),
        output: ch_tx_msgbus_to_input,
    };
    join_set_spawn(
        &mut task_set,
        "cmp_webstorage",
        task_0.spawn().map_err(super::Error::TaskMsgBusToMpsc),
    );

    // Обработка входящих сообщений
    let task_1 = tasks::Input {
        input: ch_rx_msgbus_to_input,
        output: ch_tx_input_to_output,
        storage_kind: config.storage_kind,
        fn_input: config.fn_input,
    };
    join_set_spawn(&mut task_set, "cmp_webstorage", task_1.spawn());

    // Загрузка значений из хранилища и отправка исходящих сообщений
    let task_2 = tasks::Output {
        input: ch_rx_input_to_output,
        output: ch_tx_output_to_msgbus,
        storage_kind: config.storage_kind,
        default_messages: config.default_messages,
        fn_output: config.fn_output,
    };
    join_set_spawn(&mut task_set, "cmp_webstorage", task_2.spawn());

    // Отправка исходящих сообщений
    let task_3 = shared_tasks::mpsc_to_msgbus::MpscToMsgBus {
        input: ch_rx_output_to_msgbus,
        msg_bus: msg_bus.clone(),
    };
    join_set_spawn(
        &mut task_set,
        "cmp_webstorage",
        task_3.spawn().map_err(super::Error::TaskMpscToMsgBus),
    );

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Ok(())
}
