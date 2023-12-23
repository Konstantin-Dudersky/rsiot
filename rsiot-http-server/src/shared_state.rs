use tokio::sync::mpsc;

use rsiot_extra_components::cmp_cache::CacheType;

#[derive(Clone)]
pub struct SharedState<TMessage> {
    pub output: mpsc::Sender<TMessage>,
    pub cache: CacheType<TMessage>,
}
