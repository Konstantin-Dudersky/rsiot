//! Компонент для сохранения сообщений в кеше

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::sync::mpsc;

use rsiot_messages_core::IMessage;

pub type Cache<TMessage> = Arc<Mutex<HashMap<String, TMessage>>>;

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
    cache: Cache<TMessage>,
) where
    TMessage: IMessage,
{
    while let Some(msg) = input.recv().await {
        {
            let mut lock = cache.lock().unwrap();
            lock.insert(msg.key().clone(), msg.clone());
        }
        output.send(msg).await.unwrap();
    }
}

/// Создать пустой кеш
pub fn create_cache<TMessage>() -> Cache<TMessage> {
    Arc::new(Mutex::new(HashMap::new()))
}
