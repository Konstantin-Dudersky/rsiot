//! Пример для работы с ферментером UST.
mod config;
mod message;

use tokio::main;
use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::fmt;

use rsiot_component_core::ComponentChain;
use rsiot_extra_components::cmp_logger;
use rsiot_modbus_client::cmp_modbus_client;

#[main]
async fn main() {
    // логгирование
    fmt().with_max_level(LevelFilter::INFO).init();

    let mut chain = ComponentChain::new(100)
        .add_cmp(cmp_modbus_client::new(config::config()))
        .add_cmp(cmp_logger::create(cmp_logger::Config {
            level: Level::INFO,
            header: "".into(),
        }));

    chain.spawn().await;
}
