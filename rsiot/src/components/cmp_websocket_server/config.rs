pub use crate::components_config::websocket_server::*;
use crate::message::MsgDataBound;

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
