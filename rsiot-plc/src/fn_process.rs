use std::time::Duration;

use instant::Instant;
use serde::Serialize;
use tracing::trace;

#[cfg(not(feature = "single-thread"))]
use tokio::task::spawn;
#[cfg(feature = "single-thread")]
use tokio::task::spawn_local;

#[cfg(target_arch = "wasm32")]
use gloo::timers::future::sleep;
#[cfg(not(target_arch = "wasm32"))]
use tokio::time::sleep;

use rsiot_component_core::{CmpInOut, ComponentError};
use rsiot_messages_core::MsgDataBound;

use crate::{
    config::Config,
    plc::function_block_base::{FunctionBlockBase, IFunctionBlock},
    Error,
};

type Result = std::result::Result<(), Error>;

pub async fn fn_process<TMessage, I, Q, S>(
    output: CmpInOut<TMessage>,
    config: Config<TMessage, I, Q, S>,
) -> std::result::Result<(), ComponentError>
where
    TMessage: MsgDataBound + 'static,
    I: Clone + Default + Send + Serialize + 'static + Sync,
    Q: Clone + Default + Send + Serialize + 'static + Sync,
    S: Clone + Default + Send + Serialize + 'static + Sync,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    #[cfg(feature = "single-thread")]
    let handle = spawn_local(task_main_loop::<TMessage, I, Q, S>(output, config));

    #[cfg(not(feature = "single-thread"))]
    let handle = spawn(task_main_loop::<TMessage, I, Q, S>(output, config));

    handle
        .await
        .map_err(|err| ComponentError::Execution(err.to_string()))??;
    Ok(())
}

async fn task_main_loop<TMessage, I, Q, S>(
    output: CmpInOut<TMessage>,
    config: Config<TMessage, I, Q, S>,
) -> Result
where
    TMessage: MsgDataBound + 'static,
    I: Clone + Default + Send + Serialize + Sync,
    Q: Clone + Default + Send + Serialize + Sync,
    S: Clone + Default + Send + Serialize + Sync,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    let mut fb_main = config.fb_main.clone();
    loop {
        trace!("Start PLC cycle");
        let begin = Instant::now();
        task_main::<TMessage, I, Q, S>(&output, &config, &mut fb_main).await?;
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

async fn task_main<TMessage, I, Q, S>(
    output: &CmpInOut<TMessage>,
    config: &Config<TMessage, I, Q, S>,
    fb_main: &mut FunctionBlockBase<I, Q, S>,
) -> Result
where
    TMessage: MsgDataBound + 'static,
    I: Clone + Default + Send + Serialize,
    Q: Clone + Default + Send + Serialize,
    S: Clone + Default + Send + Serialize,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    let mut input = I::default();
    {
        let cache = output.recv_cache_all().await;
        for msg in cache {
            (config.fn_input)(&mut input, &msg);
        }
    }
    fb_main.call(input);
    let msgs = (config.fn_output)(&fb_main.output);
    for msg in msgs {
        output.send_output(msg).await.map_err(Error::CmpOutput)?;
    }
    Ok(())
}
