//! Запуск;
//!
//! ```rust
//! cargo run -p rsiot-component-core2 --example multi-thread --features single-thread
//! ```

mod example_component1;
mod example_component2;
mod message;

use rsiot_component_core2::ComponentCollection;

use example_component1::{Component1, Config1};
use example_component2::{Component2, Config2};
use message::Message;
use tokio::task::LocalSet;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        let mut cmps = ComponentCollection::<Message>::new(100)
            .add_cmp(Component1::new(Config1 {}))
            .add_cmp(Component2::new(Config2 {}));

        cmps.wait_result().await.unwrap();
    });

    local_set.await;
}
