use tokio::{main, time::Duration};
use tracing::Level;
use tracing_subscriber::fmt;
use url::Url;

use rsiot_component_core::ComponentChain;
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
use rsiot_messages_core::{msg_types, ExampleMessage, ExampleMessageChannel};
use rsiot_redis_publisher::cmp_redis_publisher;

#[main]
async fn main() {
    fmt().init();

    let mut counter = 0;
    let mut chain = ComponentChain::new(100)
        .add_cmp(cmp_inject_periodic::new(cmp_inject_periodic::Config {
            period: Duration::from_secs(2),
            fn_periodic: move || {
                let msg = ExampleMessage::ValueInstantF64(msg_types::Value::new(counter as f64));

                counter += 1;
                vec![msg]
            },
        }))
        .add_cmp(cmp_logger::create(cmp_logger::Config {
            level: Level::INFO,
            header: "".into(),
        }))
        .add_cmp(cmp_redis_publisher::new(cmp_redis_publisher::Config {
            url: Url::parse("redis://127.0.0.1:6379").unwrap(),
            fn_input: |_| vec![ExampleMessageChannel::Output],
        }));

    chain.spawn().await;
}
