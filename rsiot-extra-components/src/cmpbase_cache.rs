//! Компонент для сохранения сообщений в кеше. Сохраняется только последний вариант
//!
//! Кеш представляет собой `HashMap`, а точнее `Arc<Mutex<RwLock<String, TMessage>>>`

use std::fmt::Debug;

use rsiot_component_core::{Cache, CmpInput};

#[derive(Clone, Debug)]
pub struct Config<TMessage> {
    pub cache: Cache<TMessage>,
}

pub async fn cmpbase_cache<TMsg>(mut input: CmpInput<TMsg>, config: Config<TMsg>)
where
    TMsg: Clone + Debug,
{
    while let Ok(msg) = input.recv().await {
        {
            let msg = match msg {
                Some(val) => val,
                None => continue,
            };
            let mut lock = config.cache.write().await;
            let key = msg.key.clone();
            let value = msg.clone();
            lock.insert(key, value);
        }
    }
}
