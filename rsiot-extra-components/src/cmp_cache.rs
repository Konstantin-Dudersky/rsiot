//! Компонент для сохранения сообщений в кеше. Сохраняется только последний
//! вариант
//!
//! Входящие сообщения пересылаются без изменений в исходящие
//!
//! Кеш представляет собой `HashMap`, а точнее
//! `Arc<Mutex<HashMap<String, TMessage>>>`

use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;
use tracing::error;

use rsiot_component_core::{Component, StreamInput, StreamOutput};
use rsiot_messages_core::IMessage;

pub type CacheType<TMessage> = Arc<Mutex<HashMap<String, TMessage>>>;

async fn process<TMessage>(
    input: StreamInput<TMessage>,
    output: StreamOutput<TMessage>,
    config: Config<TMessage>,
) where
    TMessage: IMessage,
{
    let mut input = match input {
        Some(val) => val,
        None => {
            let err = "Input stream not set, exit";
            error!(err);
            return;
        }
    };

    while let Some(msg) = input.recv().await {
        {
            let mut lock = config.cache.lock().await;
            let key = msg.key().clone();
            let value = msg.clone();
            lock.insert(key, value);
        }
        if let Some(output) = &output {
            output.send(msg.clone()).await.unwrap()
        }
    }
}

/// Создать пустой кеш
pub fn create_cache<TMessage>() -> CacheType<TMessage> {
    Arc::new(Mutex::new(HashMap::new()))
}

#[derive(Clone, Debug)]
pub struct Config<TMessage> {
    pub cache: CacheType<TMessage>,
}

pub fn new<TMessage>(
    config: Config<TMessage>,
) -> Box<Component<TMessage, Config<TMessage>>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(config, process);
    Box::new(cmp)
}
