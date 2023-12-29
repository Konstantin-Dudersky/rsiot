use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

pub type CacheType<TMessage> = Arc<RwLock<HashMap<String, TMessage>>>;

/// Создать пустой кеш
pub fn create_cache<TMessage>() -> CacheType<TMessage> {
    Arc::new(RwLock::new(HashMap::new()))
}
