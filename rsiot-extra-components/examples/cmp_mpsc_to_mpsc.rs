use serde::{Deserialize, Serialize};
use tokio::{main, time::Duration};

use rsiot_component_core::ComponentChain;
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger, cmp_mpsc_to_mpsc};
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
    let mut chain = ComponentChain::new(100)
        // Генерация сообщений
        .add_cmp(cmp_inject_periodic::new(cmp_inject_periodic::Config {
            period: Duration::from_secs(2),
            fn_periodic: move || {
                let msg = Message::Message0(counter);
                counter += 1.0;
                vec![msg]
            },
        }))
        // Пересылаем между каналами
        .add_cmp(cmp_mpsc_to_mpsc::create())
        // Логгирование
        .add_cmp(cmp_logger::create(cmp_logger::Config {
            level: Level::INFO,
            header: "".into(),
        }));
    chain.spawn().await;
}
