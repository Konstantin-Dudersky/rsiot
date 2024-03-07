//! Пример для работы с ферментером UST.
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
mod config;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
mod message;

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tracing::{level_filters::LevelFilter, Level};
    use tracing_subscriber::fmt;

    use rsiot::{
        component_core::{ComponentExecutor, ComponentExecutorConfig},
        components::{cmp_logger, cmp_modbus_client},
    };

    use message::Data;

    fmt().with_max_level(LevelFilter::INFO).init();

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "".into(),
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "example_single_thread".into(),
        fn_auth: |msg, _| Some(msg),
    };

    ComponentExecutor::<Data>::new(executor_config)
        .add_cmp(cmp_modbus_client::Cmp::new(config::config()))
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn main() {}
