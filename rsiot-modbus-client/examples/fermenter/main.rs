//! Пример для работы с ферментером UST.
mod config;
mod message;

use tokio::main;
use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::fmt;

use rsiot_component_core::ComponentCollection;
use rsiot_extra_components::cmp_logger;
use rsiot_modbus_client::cmp_modbus_client;

#[main]
async fn main() -> anyhow::Result<()> {
    // логгирование
    fmt().with_max_level(LevelFilter::INFO).init();

    let mut chain = ComponentCollection::new(
        100,
        vec![
            cmp_modbus_client::new(config::config()),
            cmp_logger::new(cmp_logger::Config {
                level: Level::INFO,
                header: "".into(),
            }),
        ],
    );

    chain.spawn().await?;
    Ok(())
}
