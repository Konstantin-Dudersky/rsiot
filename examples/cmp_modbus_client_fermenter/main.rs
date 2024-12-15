//! Пример для работы с ферментером UST.
#[cfg(feature = "cmp_modbus_client")]
mod config;
#[cfg(feature = "cmp_modbus_client")]
mod message;

#[cfg(feature = "cmp_modbus_client")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use tracing::{level_filters::LevelFilter, Level};
    use tracing_subscriber::fmt;

    use rsiot::{
        components::{cmp_logger, cmp_modbus_client},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::example_service::Service,
    };

    fmt().with_max_level(LevelFilter::INFO).init();

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    ComponentExecutor::new(executor_config)
        .add_cmp(cmp_modbus_client::Cmp::new(config::config()))
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(feature = "cmp_modbus_client"))]
fn main() {}
