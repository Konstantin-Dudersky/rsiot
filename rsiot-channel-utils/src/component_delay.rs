//! Перенаправление сообщений с задержкой

use std::time::Instant;

use tokio::{
    spawn,
    sync::mpsc::{self, error::SendError},
    time::{sleep, Duration},
};

use rsiot_messages_core::IMessage;
use tracing::{error, info};

use crate::{component_cache, create_cache, CacheType};

/// Перенаправление сообщений с задержкой
pub async fn component_delay<TMessage>(
    stream_input: mpsc::Receiver<TMessage>,
    stream_output: mpsc::Sender<TMessage>,
    delay: Duration,
) -> ()
where
    TMessage: IMessage + 'static,
{
    let cache = create_cache::<TMessage>();

    let _task_cache = spawn(component_cache(stream_input, None, cache.clone()));

    loop {
        info!("Component started");
        let result = loop_(cache.clone(), &stream_output, delay).await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        info!("Restarting...");
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

async fn loop_<TMessage>(
    cache: CacheType<TMessage>,
    stream_output: &mpsc::Sender<TMessage>,
    delay: Duration,
) -> Result<(), Error<TMessage>>
where
    TMessage: IMessage,
{
    loop {
        let begin = Instant::now();
        {
            let mut lock = cache.lock().await;
            for msg in lock.values() {
                stream_output.send(msg.clone()).await?;
            }
            lock.clear();
        }
        sleep(delay - begin.elapsed()).await;
    }
}
