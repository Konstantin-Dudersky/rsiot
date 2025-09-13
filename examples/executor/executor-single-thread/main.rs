//! Запуск;
//!
//! ```rust
//! cargo run -p rsiot-component-core --example single-thread --features single-thread
//! ```

#[cfg(all(feature = "single-thread", feature = "executor"))]
mod example_component1;
#[cfg(all(feature = "single-thread", feature = "executor"))]
mod example_component2;
#[cfg(all(feature = "single-thread", feature = "executor"))]
mod message;

#[cfg(all(feature = "single-thread", feature = "executor"))]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use message::Message;
    use tokio::task::LocalSet;

    use rsiot::executor::{ComponentExecutor, ComponentExecutorConfig};

    tracing_subscriber::fmt().init();

    let local_set = LocalSet::new();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        fn_auth: |_, _| None,
        delay_publish: Duration::from_secs(0),
        fn_tokio_metrics: |_| None,
    };

    local_set.spawn_local(async {
        let mut cmps = ComponentExecutor::<Message>::new(executor_config)
            .add_cmp(example_component1::Cmp::new(example_component1::Config {}))
            .add_cmp(example_component2::Cmp::new(example_component2::Config {}));

        cmps.wait_result().await?;
        Ok(()) as anyhow::Result<()>
    });

    local_set.await;

    Err(anyhow::Error::msg("Program end"))
}

#[cfg(not(all(feature = "single-thread", feature = "executor")))]
fn main() {}
