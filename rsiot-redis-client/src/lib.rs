mod component;
mod config;
mod error;
mod fn_process;

pub mod cmp_redis_client {
    pub use crate::component::Cmp;
    pub use crate::config::{Config, ConfigAlias};
}
