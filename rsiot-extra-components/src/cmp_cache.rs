//! Компонент для сохранения сообщений в кеше. Сохраняется только последний вариант
//!
//! Кеш представляет собой `HashMap`, а точнее `Arc<Mutex<HashMap<String, TMessage>>>`

use rsiot_component_core::{cache, Component, ComponentInput, ComponentOutput};
use rsiot_messages_core::IMessage;

pub use super::cmpbase_cache::{cmpbase_cache, Config};
pub use cache::{create_cache, CacheType};

async fn fn_process<TMessage>(
    mut input: ComponentInput<TMessage>,
    _output: ComponentOutput<TMessage>,
    config: Config<TMessage>,
) where
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
pub fn new<TMessage>(config: Config<TMessage>) -> Box<Component<TMessage, Config<TMessage>>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(config, fn_process);
    Box::new(cmp)
}
