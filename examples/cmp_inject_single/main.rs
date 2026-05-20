//! Запуск:
//! ```bash
//! cargo run --example cmp_inject_single
//! ```

#[cfg(feature = "executor")]
mod config_inject_single;
#[cfg(feature = "executor")]
mod config_logger;
#[cfg(feature = "executor")]
mod msg;

#[cfg(feature = "executor")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tokio::time::Duration;
    use tracing::level_filters::LevelFilter;

    use rsiot::executor::{ComponentExecutor, ComponentExecutorConfig};

    use msg::Msg;

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 10,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_secs(0),
        fn_tokio_metrics: |_| None,
    };

    ComponentExecutor::<Msg>::new(executor_config)
        .add_cmp(config_logger::cmp())
        .add_cmp(config_inject_single::cmp())
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(feature = "executor"))]
fn main() {}
