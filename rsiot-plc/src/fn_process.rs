//! Схема потока сообщений:
//!
//! ```text
//!       ----> cache
//! input       plc ------> output
//!       ---------------->
//! ```

use std::time::{Duration, Instant};

use serde::Serialize;
use tokio::{spawn, sync::mpsc, time::sleep};

use rsiot_component_core::{IComponent, StreamInput, StreamOutput};
use rsiot_extra_components::{cmp_cache, cmpbase_many_mpsc_to_mpsc, cmpbase_mpsc_to_many_mpsc};
use rsiot_messages_core::IMessage;

use crate::{
    cmp_plc::plc::function_block_base::{FunctionBlockBase, IFunctionBlock},
    config::Config,
};

pub async fn fn_process<TMessage, I, Q, S>(
    input: StreamInput<TMessage>,
    output: StreamOutput<TMessage>,
    config: Config<TMessage, I, Q, S>,
) where
    TMessage: IMessage + 'static,
    I: Clone + Default + Send + Serialize + 'static + Sync,
    Q: Clone + Default + Send + Serialize + 'static + Sync,
    S: Clone + Default + Send + Serialize + 'static + Sync,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    let (input_to_cache_tx, input_to_cache_rx) = mpsc::channel::<TMessage>(config.buffer_size);
    let (input_to_output_tx, input_to_output_rx) = mpsc::channel::<TMessage>(config.buffer_size);
    let (plc_to_output_tx, plc_to_output_rx) = mpsc::channel::<TMessage>(config.buffer_size);

    spawn(cmpbase_mpsc_to_many_mpsc::new(
        input,
        vec![Some(input_to_cache_tx), Some(input_to_output_tx)],
    ));

    spawn(cmpbase_many_mpsc_to_mpsc::new(
        vec![Some(plc_to_output_rx), Some(input_to_output_rx)],
        output,
    ));

    // кэшируем данные
    let cache = cmp_cache::create_cache::<TMessage>();
    let task_cache_config = cmp_cache::Config {
        cache: cache.clone(),
    };
    let _task_cache =
        cmp_cache::new(task_cache_config).set_and_spawn(Some(input_to_cache_rx), None);

    spawn(task_main_loop::<TMessage, I, Q, S>(
        plc_to_output_tx.clone(),
        config,
        cache.clone(),
    ));
}

async fn task_main_loop<TMessage, I, Q, S>(
    output: mpsc::Sender<TMessage>,
    config: Config<TMessage, I, Q, S>,
    cache: cmp_cache::CacheType<TMessage>,
) where
    TMessage: IMessage + 'static,
    I: Clone + Default + Send + Serialize + Sync,
    Q: Clone + Default + Send + Serialize + Sync,
    S: Clone + Default + Send + Serialize + Sync,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    let mut fb_main = config.fb_main.clone();
    loop {
        let begin = Instant::now();
        task_main::<TMessage, I, Q, S>(&output, &config, &mut fb_main, cache.clone()).await;
        let elapsed = begin.elapsed();
        let sleep_time = if config.period <= elapsed {
            Duration::from_millis(10)
        } else {
            config.period - elapsed
        };
        sleep(sleep_time).await;
    }
}

async fn task_main<TMessage, I, Q, S>(
    output: &mpsc::Sender<TMessage>,
    config: &Config<TMessage, I, Q, S>,
    fb_main: &mut FunctionBlockBase<I, Q, S>,
    cache: cmp_cache::CacheType<TMessage>,
) where
    TMessage: IMessage + 'static,
    I: Clone + Default + Send + Serialize,
    Q: Clone + Default + Send + Serialize,
    S: Clone + Default + Send + Serialize,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    let mut input = I::default();
    {
        let cache = cache.lock().await;
        for msg in cache.values() {
            (config.fn_input)(&mut input, &msg);
        }
    }
    fb_main.call(input);
    let msgs = (config.fn_output)(&fb_main.output);
    for msg in msgs {
        output.send(msg).await.unwrap();
    }
}
