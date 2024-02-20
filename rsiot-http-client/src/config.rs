pub use rsiot_components_config::http_client as config;

pub struct ConfigAlias<TMsg>(pub config::Config<TMsg>);

impl<TMsg> From<config::Config<TMsg>> for ConfigAlias<TMsg> {
    fn from(value: config::Config<TMsg>) -> Self {
        Self(value)
    }
}
