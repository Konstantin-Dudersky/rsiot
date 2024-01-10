pub use rsiot_components_config::http_client as config;
use rsiot_messages_core::IMessage;

pub struct ConfigAlias<TMsg>(pub config::Config<TMsg>)
where
    TMsg: IMessage;

impl<TMsg> From<config::Config<TMsg>> for ConfigAlias<TMsg>
where
    TMsg: IMessage,
{
    fn from(value: config::Config<TMsg>) -> Self {
        Self(value)
    }
}
