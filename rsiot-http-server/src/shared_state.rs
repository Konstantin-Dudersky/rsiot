use tokio::sync::mpsc;

use rsiot_channel_utils::CacheType;

#[derive(Clone)]
pub struct SharedState<TMessage> {
    pub cache: CacheType<TMessage>,
    pub stream_output: mpsc::Sender<TMessage>,
}
