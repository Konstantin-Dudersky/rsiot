use tokio::sync::mpsc;

use rsiot_extra_components::cmp_cache::CacheType;

#[derive(Clone)]
pub struct SharedState<TMessage> {
    pub cache: CacheType<TMessage>,
    pub stream_output: mpsc::Sender<TMessage>,
}
