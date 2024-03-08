pub use crate::components_config::redis_client::{Config, ConfigFnInputItem};
use crate::message::{IMessageChannel, MsgDataBound};

pub struct ConfigAlias<TMessage, TMessageChannel>(pub Config<TMessage, TMessageChannel>)
where
    TMessage: MsgDataBound,
    TMessageChannel: IMessageChannel;

impl<TMessage, TMessageChannel> From<Config<TMessage, TMessageChannel>>
    for ConfigAlias<TMessage, TMessageChannel>
where
    TMessage: MsgDataBound,
    TMessageChannel: IMessageChannel,
{
    fn from(value: Config<TMessage, TMessageChannel>) -> Self {
        Self(value)
    }
}
