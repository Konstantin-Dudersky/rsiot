//! Компонент для сохранения сообщений в кеше. Сохраняется только последний вариант
//!
//! Кеш представляет собой `HashMap`, а точнее `Arc<Mutex<RwLock<String, TMessage>>>`

use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use rsiot_component_core::Input;
use rsiot_messages_core::IMessage;

pub type CacheType<TMessage> = Arc<RwLock<HashMap<String, TMessage>>>;

/// Создать пустой кеш
pub fn create_cache<TMessage>() -> CacheType<TMessage> {
    Arc::new(RwLock::new(HashMap::new()))
}

#[derive(Clone, Debug)]
pub struct Config<TMessage> {
    pub cache: CacheType<TMessage>,
}

pub async fn cmpbase_cache<TMessage>(mut input: Input<TMessage>, config: Config<TMessage>)
where
    TMessage: IMessage,
{
    while let Ok(msg) = input.recv().await {
        {
            let mut lock = config.cache.write().await;
            let key = msg.key().clone();
            let value = msg.clone();
            lock.insert(key, value);
        }
    }
}
