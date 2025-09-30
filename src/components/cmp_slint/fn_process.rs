use std::time::Duration;

use futures::TryFutureExt;
use slint::ComponentHandle;
use tokio::{sync::mpsc, task::JoinSet};

use crate::{
    components::shared_tasks,
    executor::{MsgBusLinker, join_set_spawn},
    message::MsgDataBound,
};

use super::{Config, Error, Result, tasks};

pub async fn fn_process<TMainWindow, TMsg>(
    config: Config<TMsg, TMainWindow>,
    msgbus_linker: MsgBusLinker<TMsg>,
) -> Result<()>
where
    TMsg: MsgDataBound + 'static,
    TMainWindow: ComponentHandle + 'static,
{
    let buffer_size = msgbus_linker.max_capacity();

    let (ch_tx_output_to_filter, ch_rx_output_to_filter) = mpsc::channel(buffer_size);
    let (ch_tx_filter_to_msgbus, ch_rx_filter_to_msgbus) = mpsc::channel(buffer_size);

    let mut task_set = JoinSet::new();

    // Обработка входящих сообщений и изменение данных в приложении Slint
    let task = tasks::Input {
        input: msgbus_linker.input(),
        slint_window: config.slint_window.clone(),
        fn_input: config.fn_input,
    };
    join_set_spawn(&mut task_set, "cmp_slint | input", task.spawn());

    // Создание сообщений на основе взаимодествия с приложением Slint
    let task = tasks::Output {
        output: ch_tx_output_to_filter,
        slint_window: config.slint_window.clone(),
        fn_output: config.fn_output,
    };
    join_set_spawn(&mut task_set, "cmp_slint | output", task.spawn());

    // Фильтрация сообещений
    let task = shared_tasks::filter_send_periodically::FilterSendPeriodically {
        input: ch_rx_output_to_filter,
        output: ch_tx_filter_to_msgbus,
        period: Duration::from_millis(100),
    };
    join_set_spawn(
        &mut task_set,
        "cmp_slint | filter",
        task.spawn().map_err(Error::TaskFilterSendPeriodically),
    );

    // Передача сообщений в шину сообщений
    let task = shared_tasks::mpsc_to_msgbus::MpscToMsgBus {
        input: ch_rx_filter_to_msgbus,
        output: msgbus_linker.output(),
    };
    join_set_spawn(
        &mut task_set,
        "cmp_slint | mpsc_to_msgbus",
        task.spawn().map_err(Error::TaskMpscToMsgBus),
    );

    msgbus_linker.close();

    while let Some(res) = task_set.join_next().await {
        res??;
    }
    Ok(())
}
