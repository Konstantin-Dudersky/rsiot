use tokio::{sync::mpsc::channel, task::JoinSet};

use crate::{
    executor::{MsgBusLinker, join_set_spawn},
    message::*,
};

use super::{Config, tasks};

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    msgbus_linker: MsgBusLinker<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    let channel_capacity = msgbus_linker.max_capacity();

    let (ch_tx_input_to_output, ch_rx_input_to_output) = channel(channel_capacity);

    // Обработка входящих сообщений
    let task_1 = tasks::Input {
        input: msgbus_linker.input(),
        output: ch_tx_input_to_output,
        storage_kind: config.storage_kind,
        fn_input: config.fn_input,
    };
    join_set_spawn(&mut task_set, "cmp_webstorage | input", task_1.spawn());

    // Загрузка значений из хранилища и отправка исходящих сообщений
    let task_2 = tasks::Output {
        input: ch_rx_input_to_output,
        output: msgbus_linker.output(),
        storage_kind: config.storage_kind,
        default_messages: config.default_messages,
        fn_output: config.fn_output,
    };
    join_set_spawn(&mut task_set, "cmp_webstorage | output", task_2.spawn());

    msgbus_linker.close();

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Ok(())
}
