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

    let (task_msg_bus_to_mpsc_output, task_input_input) = channel(msg_bus.max_capacity());

    // Со входа компонента на задачу Input
    let task_0 = shared_tasks::msg_bus_to_mpsc::MsgBusToMpsc {
        msg_bus: msg_bus.clone(),
        output: task_msg_bus_to_mpsc_output,
    };
    join_set_spawn(
        &mut task_set,
        task_0.spawn().map_err(super::Error::TaskMsgBusToMpsc),
    );

    // Обработка входящих сообщений
    let task_1 = tasks::Input {
        input: task_input_input,
        storage_kind: config.kind,
        fn_input: config.fn_input,
    };
    join_set_spawn(&mut task_set, task_1.spawn());

    // Загрузка значений из хранилища и отправка исходящих сообщений
    let task_2 = tasks::Output {
        output: todo!(),
        storage_kind: todo!(),
        default_messages: todo!(),
        fn_output: todo!(),
    };
    join_set_spawn(&mut task_set, task_2.spawn());

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Ok(())
}
