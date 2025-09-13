#[cfg(feature = "cmp_math")]
mod config_inject_periodic;
#[cfg(feature = "cmp_math")]
mod config_math;
#[cfg(feature = "cmp_math")]
mod messages;

#[cfg(feature = "cmp_math")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tokio::time::Duration;

    use rsiot::executor::{ComponentExecutor, ComponentExecutorConfig};

    tracing_subscriber::fmt().init();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 1000,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    ComponentExecutor::new(executor_config)
        .add_cmp(config_inject_periodic::cmp())
        .add_cmp(config_math::cmp())
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(feature = "cmp_math"))]
fn main() {}
