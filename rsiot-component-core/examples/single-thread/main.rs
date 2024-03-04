//! Запуск;
//!
//! ```rust
//! cargo run -p rsiot-component-core --example single-thread --features single-thread
//! ```

mod example_component1;
mod example_component2;
mod message;

use rsiot_component_core::{ComponentExecutor, ComponentExecutorConfig};

use message::Message;
use tokio::task::LocalSet;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let local_set = LocalSet::new();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "example_single_thread".into(),
        fn_auth: |_| None,
    };

    local_set.spawn_local(async {
        let mut cmps = ComponentExecutor::<Message>::new(executor_config)
            .add_cmp(example_component1::Cmp::new(example_component1::Config {}))
            .add_cmp(example_component2::Cmp::new(example_component2::Config {}));

        cmps.wait_result().await.unwrap();
    });

    local_set.await;
}
