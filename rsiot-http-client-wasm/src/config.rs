pub use rsiot_components_config::http_client as config;
use rsiot_messages_core::MsgDataBound;

pub struct ConfigAlias<TMsg>(pub config::Config<TMsg>)
where
    TMsg: MsgDataBound;

impl<TMsg> From<config::Config<TMsg>> for ConfigAlias<TMsg>
where
    TMsg: MsgDataBound,
{
    fn from(value: config::Config<TMsg>) -> Self {
        Self(value)
    }
}
