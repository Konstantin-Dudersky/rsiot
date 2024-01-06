use std::{collections::HashMap, sync::Arc};

use futures::Future;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Кеширование сообщений
#[derive(Debug)]
pub struct Cache<TMessage>(Arc<RwLock<HashMap<String, TMessage>>>);

impl<TMessage> Cache<TMessage> {
    /// Создаем новый пустой кеш
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(HashMap::new())))
    }

    /// Блокировка кеша для чтения в синхронном коде
    pub fn blocking_read(&self) -> RwLockReadGuard<'_, HashMap<String, TMessage>> {
        self.0.blocking_read()
    }

    /// Блокировка кеша для чтения
    pub fn read(&self) -> impl Future<Output = RwLockReadGuard<'_, HashMap<String, TMessage>>> {
        self.0.read()
    }

    /// Блокировка кеша для записи
    pub fn write(&self) -> impl Future<Output = RwLockWriteGuard<'_, HashMap<String, TMessage>>> {
        self.0.write()
    }
}

impl<TMessage> Clone for Cache<TMessage> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<TMessage> Default for Cache<TMessage> {
    fn default() -> Self {
        Self::new()
    }
}
