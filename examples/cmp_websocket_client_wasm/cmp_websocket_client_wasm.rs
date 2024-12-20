//! Запуск
//!
//! ```bash
//! cargo run --example cmp_websocket_client_wasm --features="single-thread, cmp_websocket_client_wasm" --target="wasm32-unknown-unknown"
//! ```

#[cfg(feature = "cmp_websocket_client_wasm")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use rsiot::{
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_message::*, example_service::Service},
    };
    use std::time::Duration;

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    ComponentExecutor::<Custom, Service>::new(executor_config)
        .wait_result()
        .await
        .unwrap()
}

#[cfg(not(feature = "cmp_websocket_client_wasm"))]
fn main() {}
