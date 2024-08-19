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
    use rsiot::{
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::ServiceBound,
    };

    use message::Data;

    tracing_subscriber::fmt().init();

    #[allow(non_camel_case_types)]
    #[derive(Clone, Debug)]
    enum Services {
        multi_thread,
    }

    impl ServiceBound for Services {}

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Services::multi_thread,
        fn_auth: |msg, _| Some(msg),
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
