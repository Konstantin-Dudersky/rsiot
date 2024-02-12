//! Запуск;
//!
//! ```rust
//! cargo run -p rsiot-component-core --example multi-thread --features single-thread
//! ```

mod example_component1;
mod example_component2;
mod message;

use rsiot_component_core::ComponentExecutor;

use message::Message;
use tokio::task::LocalSet;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        let mut cmps = ComponentExecutor::<Message>::new(100)
            .add_cmp(example_component1::Cmp::new(example_component1::Config {}))
            .add_cmp(example_component2::Cmp::new(example_component2::Config {}));

        cmps.wait_result().await.unwrap();
    });

    local_set.await;
}
