use std::{collections::HashMap, sync::Arc};

use tokio::sync::{broadcast, mpsc, RwLock};

pub type ComponentInput<TMessage> = broadcast::Receiver<TMessage>;
pub type ComponentOutput<TMessage> = mpsc::Sender<TMessage>;
pub type CacheType<TMessage> = Arc<RwLock<HashMap<String, TMessage>>>;
