use serde::{Deserialize, Serialize};
use tokio::{main, time::Duration};
use tracing::Level;
use tracing_subscriber::fmt;
use url::Url;

use rsiot_component_core::ComponentChain;
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
use rsiot_messages_core::IMessage;
use rsiot_redis_publisher::cmp_redis_publisher;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Messages {
    Message0(u16),
}

impl IMessage for Messages {}

#[main]
async fn main() {
    fmt().init();

    let mut counter = 0;
    let mut chain = ComponentChain::new(100)
        .add_cmp(cmp_inject_periodic::new(cmp_inject_periodic::Config {
            period: Duration::from_secs(2),
            fn_periodic: move || {
                let msg = Messages::Message0(counter);
                counter += 1;
                vec![msg]
            },
        }))
        .add_cmp(cmp_logger::create(cmp_logger::Config {
            level: Level::INFO,
        }))
        .add_cmp(cmp_redis_publisher::create(cmp_redis_publisher::Config {
            url: Url::parse("redis://127.0.0.1:6379").unwrap(),
            redis_channel: "rsiot-redis-publisher".to_string(),
        }));

    chain.spawn().await;
}
