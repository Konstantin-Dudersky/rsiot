//! Перенаправление сообщений с задержкой

use std::time::Instant;

use tokio::{
    sync::mpsc::error::SendError,
    time::{sleep, Duration},
};
use tracing::{error, info};

use rsiot_component_core::{Component, IComponent, StreamInput, StreamOutput};
use rsiot_messages_core::IMessage;

use crate::cmp_cache;

async fn process<TMessage>(
    input: StreamInput<TMessage>,
    output: StreamOutput<TMessage>,
    config: Config,
) where
    TMessage: IMessage + 'static,
{
    let cache = cmp_cache::create_cache::<TMessage>();

    let _task_cache = cmp_cache::new(cmp_cache::Config {
        cache: cache.clone(),
    })
    .set_and_spawn(input, None);

    info!("Component started");
    let result = task_main(cache.clone(), output, config.delay).await;
    match result {
        Ok(_) => (),
        Err(err) => error!("{:?}", err),
    }
}

async fn task_main<TMessage>(
    cache: cmp_cache::CacheType<TMessage>,
    stream_output: StreamOutput<TMessage>,
    delay: Duration,
) -> Result<(), Error<TMessage>>
where
    TMessage: IMessage + 'static,
{
    let stream_output = match stream_output {
        Some(val) => val,
        None => return Ok(()),
    };
    loop {
        let begin = Instant::now();
        {
            let mut lock = cache.lock().await;
            for msg in lock.values() {
                stream_output.send(msg.clone()).await?;
            }
            lock.clear();
        }
        let elapsed = begin.elapsed();
        let sleep_time = if delay <= elapsed {
            Duration::from_millis(10)
        } else {
            delay - elapsed
        };
        sleep(sleep_time).await;
    }
}

#[derive(Debug)]
enum Error<TMessage> {
    SendError(SendError<TMessage>),
}

impl<TMessage> From<SendError<TMessage>> for Error<TMessage> {
    fn from(value: SendError<TMessage>) -> Self {
        Self::SendError(value)
    }
}
#[derive(Clone, Debug)]
pub struct Config {
    pub delay: Duration,
}

/// Перенаправление сообщений с задержкой
pub fn new<TMessage>(config: Config) -> Box<Component<TMessage, Config>>
where
    TMessage: IMessage + 'static + Sync,
{
    let cmp = Component::new(config, process);
    Box::new(cmp)
}
