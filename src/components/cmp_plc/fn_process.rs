use std::time::Duration;

use instant::Instant;
use serde::Serialize;
use tokio::task::JoinSet;
use tracing::trace;

#[cfg(target_arch = "wasm32")]
use gloo::timers::future::sleep;
#[cfg(not(target_arch = "wasm32"))]
use tokio::time::sleep;

use crate::{
    executor::{Cache, CmpInOut},
    message::{Message, MsgDataBound},
};

use super::{
    config::Config,
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
    let local_cache = Cache::<TMsg>::new();

    let mut task_set = JoinSet::<super::Result<()>>::new();

    // Сохранение входных сообщений в кеше
    let task = save_input_msg_in_cache(in_out.clone(), local_cache.clone());
    #[cfg(feature = "single-thread")]
    task_set.spawn_local(task);
    #[cfg(not(feature = "single-thread"))]
    task_set.spawn(task);

    // Выполнение цикла ПЛК
    let task = plc_cycle_execute_loop::<TMsg, I, Q, S>(in_out, config, local_cache);
    #[cfg(feature = "single-thread")]
    task_set.spawn_local(task);
    #[cfg(not(feature = "single-thread"))]
    task_set.spawn(task);

    while let Some(res) = task_set.join_next().await {
        res??
    }
    Ok(())
}

/// Сохранение входящих сообщений в локальном кеше
async fn save_input_msg_in_cache<TMsg>(
    mut in_out: CmpInOut<TMsg>,
    mut local_cache: Cache<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    while let Ok(msg) = in_out.recv_input().await {
        local_cache.insert(msg).await
    }
    Ok(())
}

/// Исполнение логики ПЛК в цикле
async fn plc_cycle_execute_loop<TMsg, I, Q, S>(
    in_out: CmpInOut<TMsg>,
    config: Config<TMsg, I, Q, S>,
    local_cache: Cache<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    I: Clone + Default + Send + Serialize + Sync,
    Q: Clone + Default + Send + Serialize + Sync,
    S: Clone + Default + Send + Serialize + Sync,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    let mut fb_main = config.fb_main.clone();
    let input = I::default();

    loop {
        trace!("Start PLC cycle");
        let begin = Instant::now();

        // Исполняем цикл ПЛК
        let msgs = plc_cycle_execute::<TMsg, I, Q, S>(
            &config,
            &mut fb_main,
            input.clone(),
            local_cache.clone(),
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
async fn plc_cycle_execute<TMsg, I, Q, S>(
    config: &Config<TMsg, I, Q, S>,
    fb_main: &mut FunctionBlockBase<I, Q, S>,
    mut input: I,
    local_cache: Cache<TMsg>,
) -> super::Result<Vec<Message<TMsg>>>
where
    TMsg: MsgDataBound + 'static,
    I: Clone + Default + Send + Serialize,
    Q: Clone + Default + Send + Serialize,
    S: Clone + Default + Send + Serialize,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    {
        let mut lock = local_cache.write().await;
        for msg in lock.values() {
            (config.fn_input)(&mut input, msg);
        }
        lock.clear();
    }
    fb_main.call(input);
    let msgs = (config.fn_output)(&fb_main.output);
    Ok(msgs)
}
