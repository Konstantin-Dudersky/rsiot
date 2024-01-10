pub use rsiot_components_config::websocket_client::Config;
use rsiot_messages_core::IMessage;

pub struct ConfigAlias<TMessage>(pub Config<TMessage>)
where
    TMessage: IMessage;

impl<TMessage> From<Config<TMessage>> for ConfigAlias<TMessage>
where
    TMessage: IMessage,
{
    fn from(value: Config<TMessage>) -> Self {
        ConfigAlias(value)
    }
}
