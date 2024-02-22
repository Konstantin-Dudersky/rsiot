pub use rsiot_components_config::influxdb_v2::*;

pub struct ConfigAlias<TMsg>(pub Config<TMsg>);

impl<TMsg> From<Config<TMsg>> for ConfigAlias<TMsg> {
    fn from(value: Config<TMsg>) -> Self {
        Self(value)
    }
}
