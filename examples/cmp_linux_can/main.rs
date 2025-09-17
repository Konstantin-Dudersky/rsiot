#[cfg(feature = "cmp_linux_can")]
mod config_linux_can;

#[cfg(feature = "cmp_linux_can")]
mod config_inject_periodic;

#[cfg(feature = "cmp_linux_can")]
mod messages;

#[cfg(feature = "cmp_linux_can")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use rsiot::executor::{ComponentExecutor, ComponentExecutorConfig};

    tracing_subscriber::fmt().init();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 1000,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    ComponentExecutor::new(executor_config)
        .add_cmp(config_linux_can::cmp())
        .add_cmp(config_inject_periodic::cmp())
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(feature = "cmp_linux_can"))]
fn main() {
    unimplemented!()
}
