pub use rsiot_components_config::redis_client::Config;
use rsiot_messages_core::{IMessage, IMessageChannel};

pub struct ConfigAlias<TMessage, TMessageChannel>(pub Config<TMessage, TMessageChannel>)
where
    TMessage: IMessage,
    TMessageChannel: IMessageChannel;

impl<TMessage, TMessageChannel> From<Config<TMessage, TMessageChannel>>
    for ConfigAlias<TMessage, TMessageChannel>
where
    TMessage: IMessage,
    TMessageChannel: IMessageChannel,
{
    fn from(value: Config<TMessage, TMessageChannel>) -> Self {
        Self(value)
    }
}
