use tokio::{sync::mpsc, task::JoinSet};

use crate::{
    executor::{MsgBusLinker, join_set_spawn},
    message::MsgDataBound,
};

use super::{COMPONENT_NAME, Config, Error, int_msg::IntMsg, task_edge_detect::TaskEdgeDetect};

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    msgbus_linker: MsgBusLinker<TMsg>,
) -> super::Result<()>
where
    TMsg: 'static + MsgDataBound,
{
    let (ch_tx, ch_rx) = mpsc::channel::<IntMsg>(10);

    let mut task_set: JoinSet<Result<(), Error>> = JoinSet::new();

    // Задача определения фронта сигнала
    let task = TaskEdgeDetect {
        output: ch_tx.clone(),
        pin_speed: config.pin_speed,
        pin_led: config.pin_led,
        period: config.period,
    };

    join_set_spawn(
        &mut task_set,
        format!("{COMPONENT_NAME} | task_edge_detect"),
        task.spawn(),
    );

    // Задача генерации импульсов
    let task = super::task_tick::TaskTick {
        output: ch_tx,
        period: config.period,
    };
    join_set_spawn(
        &mut task_set,
        format!("{COMPONENT_NAME} | task_tick"),
        task.spawn(),
    );

    // Задача отправки сообщений
    let task = super::task_send::TaskSend {
        input: ch_rx,
        output: msgbus_linker.output(),
        period: config.period,
        fn_output: config.fn_output,
    };
    join_set_spawn(
        &mut task_set,
        format!("{COMPONENT_NAME} | task_calculate"),
        task.spawn(),
    );

    msgbus_linker.close();

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Err(Error::TaskFnProcessEnd)
}
