pub use rsiot_components_config::websocket_server::Config;
use rsiot_messages_core::message_v2::MsgDataBound;

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
