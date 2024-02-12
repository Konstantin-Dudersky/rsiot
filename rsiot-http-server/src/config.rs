pub use rsiot_components_config::http_server::Config;
use rsiot_messages_core::IMessage;

pub struct ConfigAlias<TMsg>(pub Config<TMsg>)
where
    TMsg: IMessage;

impl<TMsg> From<Config<TMsg>> for ConfigAlias<TMsg>
where
    TMsg: IMessage,
{
    fn from(value: Config<TMsg>) -> Self {
        Self(value)
    }
}
