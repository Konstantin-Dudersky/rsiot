#[cfg(feature = "cmp_timescaledb")]
mod config_logger;
#[cfg(feature = "cmp_timescaledb")]
mod config_timescaledb_reader;
#[cfg(feature = "cmp_timescaledb")]
mod message;

#[cfg(feature = "cmp_timescaledb")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    use tokio::time::Duration;

    use rsiot::executor::{ComponentExecutor, ComponentExecutorConfig};

    let executor_config = ComponentExecutorConfig {
        buffer_size: 1000,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    ComponentExecutor::new(executor_config)
        .add_cmp(config_timescaledb_reader::cmp())
        .add_cmp(config_logger::cmp())
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(feature = "cmp_timescaledb"))]
fn main() {}
