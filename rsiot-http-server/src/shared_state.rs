use tokio::sync::mpsc;

use rsiot_component_core::CacheType;

#[derive(Clone)]
pub struct SharedState<TMessage> {
    pub output: mpsc::Sender<TMessage>,
    pub cache: CacheType<TMessage>,
}
