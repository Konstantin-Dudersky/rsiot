use std::time::Duration;

use futures::TryFutureExt;
use slint::ComponentHandle;
use tokio::{sync::mpsc, task::JoinSet};

use crate::{
    components::shared_tasks,
    executor::{join_set_spawn, CmpInOut},
    message::{MsgDataBound, ServiceBound},
};

use super::{tasks, Config, Error, Result};

pub async fn fn_process<TMainWindow, TMsg, TService>(
    config: Config<TMsg, TMainWindow>,
    msg_bus: CmpInOut<TMsg, TService>,
) -> Result<()>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    TMainWindow: ComponentHandle + 'static,
{
    let (ch_tx_msgbus_to_input, ch_rx_msgbus_to_input) = mpsc::channel(1000);
    let (ch_tx_output_to_filter, ch_rx_output_to_filter) = mpsc::channel(1000);
    let (ch_tx_filter_to_msgbus, ch_rx_filter_to_msgbus) = mpsc::channel(1000);

    let mut task_set = JoinSet::new();

    // Перенаправление входящих сообщений
    let task = shared_tasks::msgbus_to_mpsc::MsgBusToMpsc {
        msg_bus: msg_bus.clone(),
        output: ch_tx_msgbus_to_input,
    };
    join_set_spawn(&mut task_set, task.spawn().map_err(Error::TaskMsgBusToMpsc));

    // Обработка входящих сообщений и изменение данных в приложении Slint
    let task = tasks::Input {
        input: ch_rx_msgbus_to_input,
        slint_window: config.slint_window.clone(),
        fn_input: config.fn_input,
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Создание сообщений на основе взаимодествия с приложением Slint
    let task = tasks::Output {
        output: ch_tx_output_to_filter,
        slint_window: config.slint_window.clone(),
        fn_output: config.fn_output,
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Фильтрация сообещений
    let task = shared_tasks::filter_send_periodically::FilterSendPeriodically {
        input: ch_rx_output_to_filter,
        output: ch_tx_filter_to_msgbus,
        period: Duration::from_millis(100),
    };
    join_set_spawn(
        &mut task_set,
        task.spawn().map_err(Error::TaskFilterSendPeriodically),
    );

    // Передача сообщений в шину сообщений
    let task = shared_tasks::mpsc_to_msgbus::MpscToMsgBus {
        input: ch_rx_filter_to_msgbus,
        msg_bus: msg_bus.clone(),
    };
    join_set_spawn(&mut task_set, task.spawn().map_err(Error::TaskMpscToMsgBus));

    while let Some(res) = task_set.join_next().await {
        res??;
    }
    Ok(())
}
