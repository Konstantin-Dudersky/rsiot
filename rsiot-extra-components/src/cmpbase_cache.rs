//! Компонент для сохранения сообщений в кеше. Сохраняется только последний вариант
//!
//! Кеш представляет собой `HashMap`, а точнее `Arc<Mutex<RwLock<String, TMessage>>>`

use rsiot_component_core::{Cache, ComponentInput};
use rsiot_messages_core::IMessage;

#[derive(Clone, Debug)]
pub struct Config<TMessage> {
    pub cache: Cache<TMessage>,
}

pub async fn cmpbase_cache<TMessage>(mut input: ComponentInput<TMessage>, config: Config<TMessage>)
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
