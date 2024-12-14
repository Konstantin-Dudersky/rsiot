pub use crate::components_config::http_server::{Config, ConfigCmpPlcData};
use crate::message::MsgDataBound;

pub struct ConfigAlias<TMsg>(pub Config<TMsg>)
where
    TMsg: MsgDataBound;

impl<TMsg> From<Config<TMsg>> for ConfigAlias<TMsg>
where
    TMsg: MsgDataBound,
{
    fn from(value: Config<TMsg>) -> Self {
        Self(value)
    }
}
