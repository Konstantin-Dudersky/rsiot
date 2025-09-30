//! Запуск:
//! ```bash
//! cargo run --example cmp_inject_periodic
//!
//! cargo run --example cmp_inject_periodic --features single-thread
//! ```

#[cfg(feature = "executor")]
mod config_inject_periodic;
#[cfg(feature = "executor")]
mod config_logger;
#[cfg(feature = "executor")]
mod messages;

#[cfg(feature = "executor")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    use tokio::{task::LocalSet, time::Duration};
    use tracing::level_filters::LevelFilter;

    use rsiot::executor::{ComponentExecutor, ComponentExecutorConfig};

    use messages::Msg;

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 10,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_secs(0),
        fn_tokio_metrics: |_| None,
    };

    let local_set = LocalSet::new();
    local_set.spawn_local(async {
        ComponentExecutor::<Msg>::new(executor_config)
            .add_cmp(config_logger::cmp())
            .add_cmp(config_inject_periodic::cmp())
            .wait_result()
            .await?;
        Ok(()) as anyhow::Result<()>
    });
    local_set.await;

    Ok(())
}

#[cfg(not(feature = "executor"))]
fn main() {}
