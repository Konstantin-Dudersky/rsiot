use std::time::{Duration, Instant};

use serde::Serialize;
use tokio::{spawn, sync::mpsc, time::sleep};

use rsiot_component_core::{Cache, ComponentError, ComponentInput, ComponentOutput};
use rsiot_messages_core::IMessage;

use crate::{
    cmp_plc::plc::function_block_base::{FunctionBlockBase, IFunctionBlock},
    config::Config,
};

type Result<TMessage> = std::result::Result<(), Error<TMessage>>;

pub async fn fn_process<TMessage, I, Q, S>(
    _input: ComponentInput<TMessage>,
    output: ComponentOutput<TMessage>,
    config: Config<TMessage, I, Q, S>,
    cache: Cache<TMessage>,
) -> std::result::Result<(), ComponentError>
where
    TMessage: IMessage + 'static,
    I: Clone + Default + Send + Serialize + 'static + Sync,
    Q: Clone + Default + Send + Serialize + 'static + Sync,
    S: Clone + Default + Send + Serialize + 'static + Sync,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    let handle = spawn(task_main_loop::<TMessage, I, Q, S>(output, config, cache));
    handle
        .await
        .map_err(|err| ComponentError::Execution(err.to_string()))??;
    Ok(())
}

async fn task_main_loop<TMessage, I, Q, S>(
    output: ComponentOutput<TMessage>,
    config: Config<TMessage, I, Q, S>,
    cache: Cache<TMessage>,
) -> Result<TMessage>
where
    TMessage: IMessage + 'static,
    I: Clone + Default + Send + Serialize + Sync,
    Q: Clone + Default + Send + Serialize + Sync,
    S: Clone + Default + Send + Serialize + Sync,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    let mut fb_main = config.fb_main.clone();
    loop {
        let begin = Instant::now();
        task_main::<TMessage, I, Q, S>(&output, &config, &mut fb_main, cache.clone()).await?;
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
    cache: Cache<TMessage>,
) -> Result<TMessage>
where
    TMessage: IMessage + 'static,
    I: Clone + Default + Send + Serialize,
    Q: Clone + Default + Send + Serialize,
    S: Clone + Default + Send + Serialize,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    let mut input = I::default();
    {
        let cache = cache.read().await;
        for msg in cache.values() {
            (config.fn_input)(&mut input, msg);
        }
    }
    fb_main.call(input);
    let msgs = (config.fn_output)(&fb_main.output);
    for msg in msgs {
        output.send(msg).await?;
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Error<TMessage> {
    #[error("Send to channel error: {source}")]
    Send {
        #[from]
        source: tokio::sync::mpsc::error::SendError<TMessage>,
    },
}

impl<TMessage> From<Error<TMessage>> for ComponentError {
    fn from(value: Error<TMessage>) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
