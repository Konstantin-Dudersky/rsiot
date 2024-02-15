//! Пример для работы с ферментером UST.

mod config;
mod message;

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tracing::{level_filters::LevelFilter, Level};
    use tracing_subscriber::fmt;

    use rsiot_component_core::ComponentExecutor;
    use rsiot_extra_components::cmp_logger;
    use rsiot_modbus_client::cmp_modbus_client;

    use message::Messages;

    fmt().with_max_level(LevelFilter::INFO).init();

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "".into(),
    };

    ComponentExecutor::<Messages>::new(100, "fermenter")
        .add_cmp(cmp_modbus_client::Cmp::new(config::config()))
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn main() {}
