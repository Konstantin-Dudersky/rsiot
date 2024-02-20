pub use rsiot_components_config::http_server::Config;

pub struct ConfigAlias<TMsg>(pub Config<TMsg>)
where
    TMsg: Clone;

impl<TMsg> From<Config<TMsg>> for ConfigAlias<TMsg>
where
    TMsg: Clone,
{
    fn from(value: Config<TMsg>) -> Self {
        Self(value)
    }
}
