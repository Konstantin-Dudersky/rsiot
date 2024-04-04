pub use crate::components_config::timescaledb::Config;

pub struct ConfigAlias(pub Config);

impl From<Config> for ConfigAlias {
    fn from(value: Config) -> Self {
        Self(value)
    }
}
