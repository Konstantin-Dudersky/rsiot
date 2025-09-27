//! Запуск:
//! ```bash
//! cargo run --example cmp_inject_periodic
//!
//! cargo run --example cmp_inject_periodic --features single-thread
//! ```

#[cfg(feature = "executor")]
mod config_inject_periodic;

#[cfg(feature = "executor")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    use tokio::{task::LocalSet, time::Duration};
    use tracing::{Level, level_filters::LevelFilter};

    use rsiot::{
        components::cmp_logger,
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::example_message::*,
    };

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_secs(0),
        fn_tokio_metrics: |_| None,
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };

            let text = match msg {
                Custom::ValueInstantF64(content) => format!("{content}"),
                _ => return Ok(None),
            };

            Ok(Some(text))
        },
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        ComponentExecutor::<Custom>::new(executor_config)
            .add_cmp(cmp_logger::Cmp::new(logger_config))
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
