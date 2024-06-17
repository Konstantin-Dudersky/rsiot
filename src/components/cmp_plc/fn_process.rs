use std::future::Future;
use std::{sync::Arc, time::Duration};

use instant::Instant;
use serde::Serialize;
use tokio::{sync::Mutex, task::JoinSet};
use tracing::{info, trace, warn};

#[cfg(target_arch = "wasm32")]
use gloo::timers::future::sleep;
#[cfg(not(target_arch = "wasm32"))]
use tokio::time::sleep;

use crate::{
    executor::{Cache, CmpInOut},
    message::{Message, MsgDataBound},
};

use super::{
    config::{Config, ConfigRetention, ConfigRetentionRestoreResult},
    filter_identical_data::filter_identical_data,
    plc::function_block_base::{FunctionBlockBase, IFunctionBlock},
    Error,
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

    // Сохранение входных сообщений в кеше
    let task = task_save_input_msg_in_cache(in_out.clone(), input_msg_cache.clone());
    join_set_spawn(&mut task_set, task);

    // Ожидаем данные для восстановления памяти
    let retention_restore = if let Some(config_retention) = config.retention.clone() {
        let mut task_set_retention = JoinSet::<ConfigRetentionRestoreResult<S>>::new();

        // Таймаут
        // В tokio есть timeout в модуле time, но использование модуля вызывает панику в WASM.
        // task_set_retention.spawn(async move {
        //     sleep(config_retention.restore_timeout).await;
        //     ConfigRetentionRestoreResult::NoRestoreData
        // });
        let task = task_retention_timeout(config_retention.restore_timeout);
        join_set_spawn(&mut task_set_retention, task);

        let mut in_out_clone = in_out.clone();
        task_set_retention.spawn(async move {
            while let Ok(msg) = in_out_clone.recv_input().await {
                let data = (config_retention.fn_import_static)(&msg);

                let Ok(data) = data else {
                    return ConfigRetentionRestoreResult::RestoreDeserializationError;
                };

                if let Some(data) = data {
                    return ConfigRetentionRestoreResult::RestoreData(data);
                };
            }
            ConfigRetentionRestoreResult::NoRestoreData
        });

        let mut config_retention = ConfigRetentionRestoreResult::NoRestoreData;
        while let Some(task_result) = task_set_retention.join_next().await {
            config_retention = task_result?;
            task_set_retention.shutdown().await;
        }
        config_retention
    } else {
        ConfigRetentionRestoreResult::NoRestoreData
    };
    match retention_restore {
        ConfigRetentionRestoreResult::NoRestoreData => warn!("Restore retention data: no data"),
        ConfigRetentionRestoreResult::RestoreDeserializationError => {
            warn!("Restore retention data: deserialization error");
        }
        ConfigRetentionRestoreResult::RestoreData(_) => info!("Restore retention data: success"),
    }

    let fb_main = match retention_restore {
        ConfigRetentionRestoreResult::NoRestoreData => config.fb_main.clone(),
        ConfigRetentionRestoreResult::RestoreDeserializationError => config.fb_main.clone(),
        ConfigRetentionRestoreResult::RestoreData(stat) => {
            config.fb_main.clone().new_with_restore_stat(stat)
        }
    };
    let fb_main = Arc::new(Mutex::new(fb_main));

    // Выполнение цикла ПЛК
    let task = task_plc_cycle::<TMsg, I, Q, S>(
        in_out.clone(),
        config.clone(),
        fb_main.clone(),
        input_msg_cache,
    );
    join_set_spawn(&mut task_set, task);

    if let Some(config_retention) = config.retention.clone() {
        let task = task_export_i_q_s(in_out, config_retention, fb_main);
        join_set_spawn(&mut task_set, task);
    }

    while let Some(res) = task_set.join_next().await {
        res??
    }
    Ok(())
}

async fn task_retention_timeout<S>(timeout: Duration) -> ConfigRetentionRestoreResult<S>
where
    S: Clone + Default + Serialize,
{
    sleep(timeout).await;
    ConfigRetentionRestoreResult::NoRestoreData
}

/// Сохранение входящих сообщений в локальном кеше
async fn task_save_input_msg_in_cache<TMsg>(
    mut in_out: CmpInOut<TMsg>,
    mut input_msg_cache: Cache<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    while let Ok(msg) = in_out.recv_input().await {
        input_msg_cache.insert(msg).await
    }
    Ok(())
}

/// Задача bсполнения логики ПЛК в цикле
async fn task_plc_cycle<TMsg, I, Q, S>(
    in_out: CmpInOut<TMsg>,
    config: Config<TMsg, I, Q, S>,
    fb_main: Arc<Mutex<FunctionBlockBase<I, Q, S>>>,
    input_msg_cache: Cache<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    I: Clone + Default + Send + Serialize + Sync,
    Q: Clone + Default + Send + Serialize + Sync,
    S: Clone + Default + Send + Serialize + Sync,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    info!("PLC mode: STARTED");
    let mut fb_main_input = I::default();

    loop {
        trace!("Start PLC cycle");
        let begin = Instant::now();

        // Исполняем цикл ПЛК
        let msgs = plc_cycle::<TMsg, I, Q, S>(
            &config,
            fb_main.clone(),
            &mut fb_main_input,
            input_msg_cache.clone(),
        )
        .await?;

        // Записываем выходы
        for msg in msgs {
            in_out.send_output(msg).await.map_err(Error::CmpOutput)?;
        }

        let elapsed = begin.elapsed();
        trace!("End PLC cycle, elapsed: {:?}", elapsed);
        let sleep_time = if config.period <= elapsed {
            Duration::from_millis(10)
        } else {
            config.period - elapsed
        };
        sleep(sleep_time).await;
    }
}

/// Исполнение одного цикла ПЛК
async fn plc_cycle<TMsg, I, Q, S>(
    config: &Config<TMsg, I, Q, S>,
    fb_main: Arc<Mutex<FunctionBlockBase<I, Q, S>>>,
    fb_main_input: &mut I,
    input_msg_cache: Cache<TMsg>,
) -> super::Result<Vec<Message<TMsg>>>
where
    TMsg: MsgDataBound + 'static,
    I: Clone + Default + Send + Serialize,
    Q: Clone + Default + Send + Serialize,
    S: Clone + Default + Send + Serialize,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    // Инициализация структуры входов в начале цикла
    (config.fn_cycle_init)(fb_main_input);
    // Обновляем входную структуру по данным из входящих сообщений
    {
        let mut lock = input_msg_cache.write().await;
        for msg in lock.values() {
            (config.fn_input)(fb_main_input, msg);
        }
        lock.clear();
    }
    // Выполняем цикл ПЛК и формируем исходящие сообщения
    let msgs;
    {
        let mut fb_main = fb_main.lock().await;
        fb_main.call(fb_main_input.clone());
        msgs = (config.fn_output)(&fb_main.output);
    }
    Ok(msgs)
}

async fn task_export_i_q_s<TMsg, I, Q, S>(
    in_out: CmpInOut<TMsg>,
    config: ConfigRetention<TMsg, I, Q, S>,
    fb_main: Arc<Mutex<FunctionBlockBase<I, Q, S>>>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    I: Clone + Default + Send + Serialize,
    Q: Clone + Default + Send + Serialize,
    S: Clone + Default + Send + Serialize,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    loop {
        sleep(config.save_period).await;
        let input;
        let output;
        let stat;
        {
            let fb_main = fb_main.lock().await;
            input = fb_main.input.clone();
            output = fb_main.output.clone();
            stat = fb_main.stat.clone();
        }
        let msgs = (config.fn_export)(&input, &output, &stat);
        let Some(msgs) = msgs else { continue };
        for msg in msgs {
            in_out.send_output(msg).await.unwrap();
        }
    }
}

#[cfg(feature = "single-thread")]
fn join_set_spawn<F, T>(join_set: &mut JoinSet<T>, task: F)
where
    F: Future<Output = T> + 'static,
    T: Send + 'static,
{
    join_set.spawn_local(task);
}

#[cfg(not(feature = "single-thread"))]
fn join_set_spawn<F, T>(join_set: &mut JoinSet<T>, task: F)
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    join_set.spawn(task);
}
