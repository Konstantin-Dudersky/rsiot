mod config;
mod errors;
mod new;
mod process;
mod types;

#[doc = include_str!("../doc/component-modbus-client.svg")]
pub mod cmp_modbus_client {
    pub use crate::new::new;
    pub use rsiot_components_config::modbus_client::*;
}
