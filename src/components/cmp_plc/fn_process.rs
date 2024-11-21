use std::sync::Arc;

use futures::TryFutureExt;
use serde::Serialize;
use tokio::{sync::mpsc, sync::Mutex, task::JoinSet};
use tracing::info;

use crate::{
    components::shared_tasks,
    executor::{join_set_spawn, Cache, CmpInOut},
    message::MsgDataBound,
};

use super::{
    config::Config,
    plc::{FunctionBlockBase, IFunctionBlock},
    tasks, Error,
};

pub async fn fn_process<TMsg, I, Q, S>(
    in_out: CmpInOut<TMsg>,
    config: Config<TMsg, I, Q, S>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    I: Clone + Default + Send + Serialize + 'static + Sync,
    Q: Clone + Default + Send + Serialize + 'static + Sync,
    S: Clone + Default + Send + Serialize + 'static + Sync,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    info!("PLC mode: STOPPED");

    let input_msg_cache = Cache::<TMsg>::new();

    let mut task_set = JoinSet::<super::Result<()>>::new();

    let buffer_size = in_out.max_capacity();
    let (channel_plc_to_filter_send, channel_plc_to_filter_recv) = mpsc::channel(buffer_size);
    let (channel_filter_to_output_send, channel_filter_to_output_recv) = mpsc::channel(buffer_size);

    // Ожидаем данные для восстановления памяти
    let fb_main = tasks::Retention {
        cmp_in_out: in_out.clone(),
        config_retention: config.retention.clone(),
        fb_main: config.fb_main.clone(),
    }
    .spawn()
    .await?;
    let fb_main = Arc::new(Mutex::new(fb_main));

    // Сохранение входных сообщений в кеше
    let task = tasks::SaveInputInCache {
        in_out: in_out.clone(),
        input_msg_cache: input_msg_cache.clone(),
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Выполнение цикла ПЛК
    let task = tasks::PlcLoop {
        input_msg_cache,
        output: channel_plc_to_filter_send,
        config: config.clone(),
        fb_main: fb_main.clone(),
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Фильтрация исходящих сообщений
    let task = shared_tasks::filter_identical_data::FilterIdenticalData {
        input: channel_plc_to_filter_recv,
        output: channel_filter_to_output_send,
    };
    join_set_spawn(
        &mut task_set,
        task.spawn().map_err(Error::FilterMsgsWithSameData),
    );

    // Пересылка сообщений на выход компонента
    let task = shared_tasks::mpsc_to_msgbus::MpscToMsgBus {
        input: channel_filter_to_output_recv,
        cmp_in_out: in_out.clone(),
    };
    join_set_spawn(&mut task_set, task.spawn().map_err(Error::ToCmpOutput));

    // Периодический экспорт состояния
    let task = tasks::ExportCurrentState {
        in_out: in_out.clone(),
        config_retention: config.retention,
        fb_main: fb_main.clone(),
    };
    join_set_spawn(&mut task_set, task.spawn());

    while let Some(res) = task_set.join_next().await {
        res??
    }
    Ok(())
}
