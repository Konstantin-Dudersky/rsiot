//! Запуск:
//!
//! ```rust
//! cargo run --example cmp_external_fn_process --features single-thread
//!
//! cargo run --example cmp_external_fn_process
//! ```

mod config_external_fn_process;
mod config_inject_periodic;
mod messages;

#[cfg(feature = "executor")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use std::time::Duration;

    use tokio::task::LocalSet;
    use tracing::level_filters::LevelFilter;

    use rsiot::executor::{ComponentExecutor, ComponentExecutorConfig};

    use crate::messages::*;

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_secs(0),
        fn_tokio_metrics: |_| None,
    };

    let task_set = LocalSet::new();
    task_set.spawn_local(async move {
        ComponentExecutor::<Msg>::new(executor_config)
            .add_cmp(config_external_fn_process::cmp())
            .add_cmp(config_inject_periodic::cmp())
            .wait_result()
            .await
    });
    task_set.await;
}

#[cfg(not(feature = "executor"))]
fn main() {}
