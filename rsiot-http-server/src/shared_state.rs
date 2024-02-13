use rsiot_messages_core::IMessage;
use tokio::sync::mpsc;

use crate::Config;
use rsiot_component_core::Cache;

#[derive(Clone)]
pub struct SharedState<TMsg>
where
    TMsg: IMessage,
{
    pub output: mpsc::Sender<TMsg>,
    pub cache: Cache<TMsg>,
    pub config: Config<TMsg>,
}
