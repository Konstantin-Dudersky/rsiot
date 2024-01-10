pub use rsiot_components_config::timescaledb_storing::Config;

pub struct ConfigAlias(pub Config);

impl From<Config> for ConfigAlias {
    fn from(value: Config) -> Self {
        Self(value)
    }
}
