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
        message::example_message::*,
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "cmp_websocket_client_wasm_example".into(),
        fn_auth: |msg, _| Some(msg),
    };

    ComponentExecutor::<Custom>::new(executor_config)
        .wait_result()
        .await
        .unwrap()
}

#[cfg(not(feature = "cmp_websocket_client_wasm"))]
fn main() {}
