use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::CacheType;

/// Создать пустой кеш
pub fn create_cache<TMessage>() -> CacheType<TMessage> {
    Arc::new(RwLock::new(HashMap::new()))
}
