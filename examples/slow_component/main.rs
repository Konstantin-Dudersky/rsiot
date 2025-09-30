#[cfg(feature = "executor")]
mod config_external_fn_process;
#[cfg(feature = "executor")]
mod config_inject_periodic;
#[cfg(feature = "executor")]
mod messages;

#[cfg(feature = "executor")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use rsiot::executor::{ComponentExecutor, ComponentExecutorConfig};

    use messages::*;

    tracing_subscriber::fmt().init();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 20,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    ComponentExecutor::<Msg>::new(executor_config)
        .add_cmp(config_inject_periodic::cmp())
        .add_cmp(config_external_fn_process::cmp())
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(feature = "executor"))]
fn main() {}
