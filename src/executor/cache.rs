use std::{collections::HashMap, sync::Arc};

use futures::Future;
use tokio::sync::{Mutex, MutexGuard};

use crate::message::{Message, MsgDataBound};

type Hash<TMsg> = HashMap<String, Message<TMsg>>;

/// Кеш сообщений
#[derive(Debug)]
pub struct Cache<TMsg>(Arc<Mutex<Hash<TMsg>>>);

impl<TMsg> Cache<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Создаем новый пустой кеш
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }

    /// Блокировка кеша для чтения в синхронном коде
    pub fn blocking_read(&self) -> MutexGuard<'_, Hash<TMsg>> {
        self.0.blocking_lock()
    }

    /// Блокировка кеша для чтения
    pub fn read(&self) -> impl Future<Output = MutexGuard<'_, Hash<TMsg>>> {
        self.0.lock()
    }

    /// Блокировка кеша для записи
    pub fn write(&self) -> impl Future<Output = MutexGuard<'_, Hash<TMsg>>> {
        self.0.lock()
    }

    /// Очистить кеш
    pub async fn clear(&mut self) {
        let mut lock = self.0.lock().await;
        lock.clear()
    }

    /// Вставить сообщение в кеш
    pub async fn insert(&mut self, msg: Message<TMsg>) {
        let mut lock = self.0.lock().await;
        let key = msg.key.clone();
        lock.insert(key, msg);
    }
}

impl<TMessage> Clone for Cache<TMessage> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<TMessage> Default for Cache<TMessage>
where
    TMessage: MsgDataBound,
{
    fn default() -> Self {
        Self::new()
    }
}
