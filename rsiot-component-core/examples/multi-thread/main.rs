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
    use rsiot_component_core::ComponentExecutor;

    use message::Message;

    tracing_subscriber::fmt().init();

    ComponentExecutor::<Message>::new(100, "multi-thread")
        .add_cmp(example_component1::Cmp::new(example_component1::Config {}))
        .add_cmp(example_component2::Cmp::new(example_component2::Config {}))
        .wait_result()
        .await
        .unwrap();
}
