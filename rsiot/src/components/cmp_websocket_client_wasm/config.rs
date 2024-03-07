pub use crate::components::components_config::websocket_client::Config;
use rsiot_messages_core::MsgDataBound;

pub struct ConfigAlias<TMessage>(pub Config<TMessage>)
where
    TMessage: MsgDataBound;

impl<TMessage> From<Config<TMessage>> for ConfigAlias<TMessage>
where
    TMessage: MsgDataBound,
{
    fn from(value: Config<TMessage>) -> Self {
        Self(value)
    }
}
