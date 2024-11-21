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

    let (taskout_msgbus_to_mpsc, taskin_input) = channel(channel_capacity);
    let (taskout_output, taskin_mpsc_to_msgbus) = channel(channel_capacity);

    // Со входа компонента на задачу Input
    let task_0 = shared_tasks::msgbus_to_mpsc::MsgBusToMpsc {
        msg_bus: msg_bus.clone(),
        output: taskout_msgbus_to_mpsc,
    };
    join_set_spawn(
        &mut task_set,
        task_0.spawn().map_err(super::Error::TaskMsgBusToMpsc),
    );

    // Обработка входящих сообщений
    let task_1 = tasks::Input {
        input: taskin_input,
        storage_kind: config.storage_kind,
        fn_input: config.fn_input,
    };
    join_set_spawn(&mut task_set, task_1.spawn());

    // Загрузка значений из хранилища и отправка исходящих сообщений
    let task_2 = tasks::Output {
        output: taskout_output,
        storage_kind: config.storage_kind,
        default_messages: config.default_messages,
        fn_output: config.fn_output,
    };
    join_set_spawn(&mut task_set, task_2.spawn());

    // Отправка исходящих сообщений
    let task_3 = shared_tasks::mpsc_to_msgbus::MpscToMsgBus {
        input: taskin_mpsc_to_msgbus,
        cmp_in_out: msg_bus.clone(),
    };
    join_set_spawn(
        &mut task_set,
        task_3.spawn().map_err(super::Error::TaskMpscToMsgBus),
    );

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Ok(())
}
