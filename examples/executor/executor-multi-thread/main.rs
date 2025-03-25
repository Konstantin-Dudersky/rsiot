//! Запуск;
//!
//! ```rust
//! cargo run -p rsiot-component-core --example multi-thread
//! ```

#[cfg(all(not(feature = "single-thread"), feature = "executor"))]
mod example_component1;
#[cfg(all(not(feature = "single-thread"), feature = "executor"))]
mod example_component2;
#[cfg(all(not(feature = "single-thread"), feature = "executor"))]
mod message;

#[cfg(all(not(feature = "single-thread"), feature = "executor"))]
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    use std::time::Duration;

    use rsiot::executor::{ComponentExecutor, ComponentExecutorConfig};

    use message::Data;

    tracing_subscriber::fmt().init();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    ComponentExecutor::<Data>::new(executor_config)
        .add_cmp(example_component1::Cmp::new(example_component1::Config {}))
        .add_cmp(example_component2::Cmp::new(example_component2::Config {}))
        .wait_result()
        .await
        .unwrap();
}

#[cfg(not(all(not(feature = "single-thread"), feature = "executor")))]
fn main() {}
