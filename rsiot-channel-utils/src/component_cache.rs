//! Компонент для сохранения сообщений в кеше

use std::{collections::HashMap, sync::Arc};

use tokio::sync::{mpsc, Mutex};

use rsiot_messages_core::IMessage;

pub type CacheType<TMessage> = Arc<Mutex<HashMap<String, TMessage>>>;

/// Компонент для сохранения сообщений в кеше. Сохраняется только последний
/// вариант
///
/// Входящие сообщения пересылаются без изменений в исходящие
///
/// Кеш представляет собой `HashMap`, а точнее
/// `Arc<Mutex<HashMap<String, TMessage>>>`
pub async fn component_cache<TMessage>(
    mut input: mpsc::Receiver<TMessage>,
    output: mpsc::Sender<TMessage>,
    cache: CacheType<TMessage>,
) where
    TMessage: IMessage,
{
    while let Some(msg) = input.recv().await {
        {
            let mut lock = cache.lock().await;
            lock.insert(msg.key().clone(), msg.clone());
        }
        output.send(msg).await.unwrap();
    }
}

/// Создать пустой кеш
pub fn create_cache<TMessage>() -> CacheType<TMessage> {
    Arc::new(Mutex::new(HashMap::new()))
}
