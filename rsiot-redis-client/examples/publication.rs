//! Запуск:
//!
//! ```bash
//! cargo run -p rsiot-redis-client --example publication
//! ```

use tokio::{main, time::Duration};
use tracing::Level;
use tracing_subscriber::fmt;
use url::Url;

use rsiot_component_core::ComponentCollection;
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
use rsiot_messages_core::{msg_types, ExampleMessage, ExampleMessageChannel};
use rsiot_redis_client::cmp_redis_client;

#[main]
async fn main() {
    fmt().init();

    let mut counter = 0;
    let mut chain = ComponentCollection::new(
        100,
        vec![
            cmp_inject_periodic::new(cmp_inject_periodic::Config {
                period: Duration::from_secs(2),
                fn_periodic: move || {
                    let msg =
                        ExampleMessage::ValueInstantF64(msg_types::Value::new(counter as f64));

                    counter += 1;
                    vec![msg]
                },
            }),
            cmp_logger::new(cmp_logger::Config {
                level: Level::INFO,
                header: "".into(),
            }),
            cmp_redis_client::new(cmp_redis_client::Config {
                url: Url::parse("redis://127.0.0.1:6379").unwrap(),
                fn_input: |_| vec![ExampleMessageChannel::Output],
                subscription_channel: ExampleMessageChannel::Output,
            }),
        ],
    );

    chain.spawn().await;
}
