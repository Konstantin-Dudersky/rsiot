use serde::{Deserialize, Serialize};
use tokio::{main, time::Duration};

use rsiot_channel_utils::{cmp_inject_periodic, cmp_logger};
use rsiot_component_core::ComponentChain;
use rsiot_messages_core::IMessage;
use tracing::Level;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Message {
    Message0(f64),
    Message1(f64),
    CombineMessage(f64, f64),
}

impl IMessage for Message {}

#[main]
async fn main() {
    tracing_subscriber::fmt().init();

    let mut counter = 0.0;
    let mut chain = ComponentChain::<Message>::init(100)
        .start_cmp(cmp_inject_periodic::create(cmp_inject_periodic::Config {
            period: Duration::from_secs(2),
            fn_periodic: move || {
                let msg = Message::Message0(counter);
                counter += 1.0;
                vec![msg]
            },
        }))
        .end_cmp(cmp_logger::create(cmp_logger::Config {
            level: Level::INFO,
        }));
    chain.spawn().await;
}
