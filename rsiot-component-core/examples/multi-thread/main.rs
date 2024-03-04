//! Запуск;
//!
//! ```rust
//! cargo run -p rsiot-component-core --example multi-thread
//! ```

#[cfg(any(
    feature = "single-thread",
    not(any(target_arch = "x86_64", target_arch = "aarch64"))
))]
fn main() {
    unimplemented!()
}

#[cfg(not(feature = "single-thread"))]
mod example_component1;
#[cfg(not(feature = "single-thread"))]
mod example_component2;
#[cfg(not(feature = "single-thread"))]
mod message;

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[cfg(not(feature = "single-thread"))]
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    use rsiot_component_core::{ComponentExecutor, ComponentExecutorConfig};

    use message::Data;

    tracing_subscriber::fmt().init();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "multi-thread".into(),
        fn_auth: |_| None,
    };

    ComponentExecutor::<Data>::new(executor_config)
        .add_cmp(example_component1::Cmp::new(example_component1::Config {}))
        .add_cmp(example_component2::Cmp::new(example_component2::Config {}))
        .wait_result()
        .await
        .unwrap();
}
