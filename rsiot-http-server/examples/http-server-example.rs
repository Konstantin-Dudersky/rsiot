use serde::{Deserialize, Serialize};
use tokio::{main, time::Duration};
use tracing::Level;
use tracing_subscriber::filter::LevelFilter;

use rsiot_component_core::ComponentChain;
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
use rsiot_http_server::cmp_http_server;
use rsiot_messages_core::IMessage;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Message {
    Message0(f64),
    Message1(f64),
    CombineMessage(f64, f64),
}

impl IMessage for Message {
    fn into_eav(self) -> Vec<rsiot_messages_core::eav::EavModel> {
        todo!()
    }
}

#[main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let mut counter = 0.0;

    let mut chain = ComponentChain::new(100)
        .add_cmp(cmp_inject_periodic::new(cmp_inject_periodic::Config {
            period: Duration::from_secs(2),
            fn_periodic: move || {
                let msg1 = Message::Message0(counter);
                let msg2 = Message::Message1(counter * 2.0);
                counter += 1.0;
                vec![msg1, msg2]
            },
        }))
        .add_cmp(cmp_http_server::new(cmp_http_server::Config { port: 8011 }))
        .add_cmp(cmp_logger::new(cmp_logger::Config {
            level: Level::INFO,
            header: "".into(),
        }));

    chain.spawn().await;
}
